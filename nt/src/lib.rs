use core::panic;
use std::{
    collections::HashMap,
    marker::PhantomData,
    sync::{
        atomic::{AtomicI64, AtomicU32},
        Arc, Mutex,
    },
    time::Duration,
};

use flume::{unbounded, Receiver, RecvError, Sender};
use futures::{select, Future, FutureExt};
use payload::Payload;
use thiserror::Error;
use time::get_time;
use types::{BinaryData, BinaryMessage, Properties, SubscriptionOptions, TextMessage};

pub mod payload;
pub mod time;
#[cfg(feature = "tokio")]
pub mod tokio;
pub mod types;
#[cfg(feature = "wasm")]
pub mod wasm;

/// Any type of message that could be sent or recieved from a websocket
#[derive(Debug)]
pub enum NtMessage {
    Text(TextMessage),
    Binary(BinaryMessage),
}

struct Topics {
    topics: HashMap<String, Sender<SubscriberUpdate>>,
    topic_ids: HashMap<u32, String>,
}

impl Default for Topics {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            topic_ids: Default::default(),
        }
    }
}

struct InnerNetworkTableClient {
    receive: Receiver<Result<NtMessage>>,
    send: Sender<NtMessage>,
    topics: Mutex<Topics>,
    time_offset: AtomicI64,

    subuid: AtomicU32,
    pubuid: AtomicU32,
}

enum SubscriberUpdate {
    Properties(Properties),
    Data(BinaryData),
    Type(String),
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error while sending a message")]
    Send,
    #[error("Error while receiving a message: {0}")]
    Receive(#[from] RecvError),
    #[error("Other error occured: {0}")]
    Other(Box<dyn std::error::Error + 'static + Send>),
    #[error("Encountered an unknown frame")]
    UnknownFrame,
    #[error("Encountered an incorrect type")]
    Type,
}

type Result<T, E = Error> = std::result::Result<T, E>;

/// A generic timer driver
pub trait Timer {
    /// Delay for the specified duration
    fn time(duration: Duration) -> impl std::future::Future<Output = ()> + Send;
}

/// A generic backend that a client can use. [backend::TokioBackend] is a good example.
pub trait Backend {
    /// A type like a join handle that whatever is using the client might need
    type Output;
    type Error: std::error::Error + 'static + Send;

    /// Using the hostname and client name create a backend that sends [NtMessage] or [Error] to
    /// the client and passes on [NtMessage] to the server
    fn create(
        host: &str,
        name: &str,
        send: Sender<Result<NtMessage>>,
        receive: Receiver<NtMessage>,
    ) -> std::result::Result<Self::Output, Self::Error>;
}

impl InnerNetworkTableClient {
    async fn new<B: Backend>(host: &str, name: &str) -> Result<(Self, B::Output)> {
        let (send_out, receive_out) = unbounded();
        let (send_in, receive_in) = unbounded();

        let out = match B::create(host, name, send_in, receive_out) {
            Ok(out) => out,
            Err(err) => return Err(Error::Other(Box::new(err))),
        };

        send_out
            .send(NtMessage::Binary(BinaryMessage {
                id: -1,
                timestamp: 0,
                data: BinaryData::Int(get_time() as i64),
            }))
            .map_err(|_| Error::Send)?;

        let NtMessage::Binary(msg) = receive_in.recv_async().await?? else {
            return Err(Error::Type);
        };

        if msg.id != -1 {
            return Err(Error::Type); // TODO: Maybe not the right response
        }

        let BinaryData::Int(time) = msg.data else {
            return Err(Error::Type);
        };

        let server_time = (get_time() as i64 - time) / 2 + msg.timestamp as i64;
        let offset = server_time - get_time() as i64;

        Ok((
            Self {
                send: send_out,
                receive: receive_in,
                topics: Mutex::new(Default::default()),
                time_offset: AtomicI64::new(offset),

                subuid: AtomicU32::new(u32::MIN),
                pubuid: AtomicU32::new(u32::MIN),
            },
            out,
        ))
    }

    fn get_server_time(&self) -> u64 {
        let offset = self.time_offset.load(std::sync::atomic::Ordering::Relaxed);
        (get_time() as i64 + offset) as u64
    }

    async fn main_loop<T: Timer>(&self) -> Result<()> {
        select! {
            res = self.time_loop::<T>().fuse() => {
                return res;
            }
            res = self.recv_loop().fuse() => {
                return res;
            }
        }
    }

    async fn time_loop<T: Timer>(&self) -> Result<()> {
        loop {
            T::time(Duration::from_secs(2)).await;

            self.start_sync_time()?;
        }
    }

    async fn recv_loop(&self) -> Result<()> {
        loop {
            let val = self.receive.recv_async().await??;

            match val {
                NtMessage::Text(msg) => match msg {
                    TextMessage::Announce {
                        name,
                        id,
                        data_type,
                        pubuid: _,
                        properties,
                    } => {
                        let mut topics = self.topics.lock().unwrap();

                        let Some(sender) = topics.topics.get(&name) else {
                            continue;
                        };

                        if sender.send(SubscriberUpdate::Type(data_type)).is_err()
                            || sender
                                .send(SubscriberUpdate::Properties(properties))
                                .is_err()
                        {
                            topics.topics.remove(&name);
                        } else {
                            topics.topic_ids.insert(id, name);
                        }
                    }
                    TextMessage::Unannounce { name, id } => {
                        let mut topics = self.topics.lock().unwrap();

                        topics.topics.remove(&name);
                        topics.topic_ids.remove(&id);
                    }
                    TextMessage::Properties {
                        name,
                        ack: _,
                        update,
                    } => {
                        let mut topics = self.topics.lock().unwrap();

                        let topic = topics.topics.get(&name);

                        if let Some(topic) = topic {
                            if topic.send(SubscriberUpdate::Properties(update)).is_err() {
                                topics.topics.remove(&name);
                            }
                        }
                    }
                    _ => unreachable!("A server-bound message was sent to the client"),
                },
                NtMessage::Binary(msg) => {
                    if msg.id == -1 {
                        let BinaryData::Int(time) = msg.data else {
                            return Err(Error::Type);
                        };

                        let server_time = (get_time() as i64 - time) / 2 + msg.timestamp as i64;
                        let offset = server_time - get_time() as i64;

                        self.time_offset
                            .fetch_min(offset, std::sync::atomic::Ordering::Relaxed);
                    } else {
                        let mut topics = self.topics.lock().unwrap();

                        let Some(name) = topics.topic_ids.get(&(msg.id as u32)) else {
                            topics.topic_ids.remove(&(msg.id as u32));
                            continue;
                        };

                        let is_sender_dropped = topics
                            .topics
                            .get(name)
                            .map(|topic| topic.send(SubscriberUpdate::Data(msg.data)).is_err())
                            .unwrap_or(false);

                        if is_sender_dropped {
                            let name = name.to_owned();
                            topics.topics.remove(&name);
                            topics.topic_ids.remove(&(msg.id as u32));
                        }
                    }
                }
            }
        }
    }

    fn start_sync_time(&self) -> Result<()> {
        self.send
            .send(NtMessage::Binary(BinaryMessage {
                id: -1,
                timestamp: 0,
                data: BinaryData::Int(get_time() as i64),
            }))
            .map_err(|_| Error::Send)?;

        Ok(())
    }

    fn subscribe(&self, topics: Vec<String>, options: SubscriptionOptions) -> Result<u32> {
        let id = self.new_subuid();
        self.send
            .send(NtMessage::Text(TextMessage::Subscribe {
                topics,
                subuid: id,
                options,
            }))
            .map_err(|_| Error::Send)?;

        Ok(id)
    }

    fn unsubscribe(&self, id: u32) -> Result<()> {
        self.send
            .send(NtMessage::Text(TextMessage::Unsubscribe { subuid: id }))
            .map_err(|_| Error::Send)?;

        Ok(())
    }

    fn publish(&self, name: String, data_type: String, properties: Properties) -> Result<u32> {
        let id = self.new_pubuid();

        self.send
            .send(NtMessage::Text(TextMessage::Publish {
                name,
                pubuid: id,
                data_type,
                properties,
            }))
            .map_err(|_| Error::Send)?;

        Ok(id)
    }

    fn unpublish(&self, id: u32) -> Result<()> {
        self.send
            .send(NtMessage::Text(TextMessage::Unpublish { pubuid: id }))
            .map_err(|_| Error::Send)?;

        Ok(())
    }

    fn new_subuid(&self) -> u32 {
        self.subuid
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    fn new_pubuid(&self) -> u32 {
        self.pubuid
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}

// An instance of a network table client. Based on an [Arc] internally, so cheap to copy
#[derive(Clone)]
pub struct NetworkTableClient {
    inner: Arc<InnerNetworkTableClient>,
}

impl NetworkTableClient {
    /// Create a new client using the hostname, client name, and a backend type
    pub async fn new<B: Backend>(host: &str, name: &str) -> Result<(Self, B::Output)> {
        let (inner, out) = InnerNetworkTableClient::new::<B>(host, name).await?;

        Ok((
            Self {
                inner: Arc::new(inner),
            },
            out,
        ))
    }

    /// This returns a future that should be run on the side, usually, in an async task. This
    /// future must remain alive for as long as subscriber and time updates are required
    pub fn main_task<T: Timer>(&self) -> impl Future<Output = Result<()>> + 'static {
        let inner = self.inner.clone();

        async move { inner.main_loop::<T>().await }
    }

    /// Create a subscriber for a topic with a certain payload type
    pub fn subscribe<P: Payload>(&self, name: String) -> Result<Subscriber<P>> {
        let (sender, receiver) = unbounded();

        self.inner
            .topics
            .lock()
            .unwrap()
            .topics
            .insert(name.clone(), sender);

        let id = self
            .inner
            .subscribe(vec![name.clone()], Default::default())?;

        Ok(Subscriber {
            name,
            properties: None,
            input: receiver,
            id,
            client: self.inner.clone(),
            phantom: PhantomData,
        })
    }

    /// Create a publisher for a topic with a certain payload type
    pub fn publish<P: Payload>(&self, name: String) -> Result<Publisher<P>> {
        let id = self
            .inner
            .publish(name.clone(), P::name().to_owned(), Default::default())?;

        Ok(Publisher {
            name,
            id,
            client: self.inner.clone(),
            phantom: PhantomData,
        })
    }
}

pub struct Subscriber<P: Payload> {
    name: String,
    properties: Option<Properties>,
    input: Receiver<SubscriberUpdate>,
    id: u32,
    client: Arc<InnerNetworkTableClient>,
    phantom: PhantomData<P>,
}

impl<P: Payload> Subscriber<P> {
    fn consume_updates(&mut self) -> Result<Option<P>> {
        let mut data = None;
        for update in self.input.drain() {
            match update {
                SubscriberUpdate::Properties(props) => {
                    if self.properties.is_none() {
                        self.properties = Some(Default::default());
                    }

                    self.properties.as_mut().unwrap().update(props);
                }
                SubscriberUpdate::Data(bin_data) => {
                    data = Some(P::parse(bin_data).map_err(|_| Error::Type)?);
                }
                SubscriberUpdate::Type(val) => {
                    if &val != P::name() {
                        return Err(Error::Type);
                    }
                }
            }
        }

        Ok(data)
    }

    /// Wait for a new payload value to become avaliable
    pub async fn get(&mut self) -> Result<P> {
        if !self.input.is_empty() {
            if let Some(val) = self.consume_updates()? {
                return Ok(val);
            }
        }

        loop {
            let val = self.input.recv_async().await?;

            match val {
                SubscriberUpdate::Properties(props) => {
                    if self.properties.is_none() {
                        self.properties = Some(Default::default());
                    }

                    self.properties.as_mut().unwrap().update(props);
                }
                SubscriberUpdate::Data(val) => {
                    break P::parse(val).map_err(|_| Error::Type);
                }
                SubscriberUpdate::Type(ty) => {
                    if &ty != P::name() {
                        return Err(Error::Type);
                    }
                }
            }
        }
    }

    pub fn properties(&self) -> Option<&Properties> {
        self.properties.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<P: Payload> Drop for Subscriber<P> {
    fn drop(&mut self) {
        let _ = self.client.unsubscribe(self.id);
    }
}

pub struct Publisher<P: Payload> {
    name: String,
    id: u32,
    client: Arc<InnerNetworkTableClient>,
    phantom: PhantomData<P>,
}

impl<P: Payload> Publisher<P> {
    pub fn set(&self, value: P) -> Result<()> {
        self.client
            .send
            .send(NtMessage::Binary(BinaryMessage {
                id: self.id as i64,
                timestamp: self.client.get_server_time(),
                data: value.to_val(),
            }))
            .map_err(|_| Error::Send)
    }

    pub fn set_properties(&self, props: Properties) -> Result<()> {
        self.client
            .send
            .send(NtMessage::Text(TextMessage::SetProperties {
                name: self.name.clone(),
                update: props,
            }))
            .map_err(|_| Error::Send)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<P: Payload> Drop for Publisher<P> {
    fn drop(&mut self) {
        let _ = self.client.unpublish(self.id);
    }
}
