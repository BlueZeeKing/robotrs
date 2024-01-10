use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicI64, AtomicU32, AtomicU64},
        Mutex,
    },
    time::Duration,
};

use flume::{unbounded, Receiver, Sender};
use futures::{select, FutureExt};

use crate::{
    time::get_time,
    types::{BinaryData, BinaryMessage, Properties, SubscriptionOptions, TextMessage},
    Backend, Error, NtMessage, Result, SubscriberUpdate, Timer,
};

pub struct Topics {
    pub topics: HashMap<String, Sender<SubscriberUpdate>>,
    pub topic_ids: HashMap<u32, String>,

    pub prefix_channels: HashMap<String, Sender<(String, Receiver<SubscriberUpdate>)>>,

    pub publishers: HashMap<u32, PublisherData>,
    pub subscribers: HashMap<u32, SubscriberData>,
}

pub struct PublisherData {
    pub name: String,
    pub data_type: String,
}

pub struct SubscriberData {
    pub name: String,
    pub options: SubscriptionOptions,
}

impl Default for Topics {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            topic_ids: Default::default(),

            prefix_channels: Default::default(),

            publishers: Default::default(),
            subscribers: Default::default(),
        }
    }
}

pub struct InnerNetworkTableClient {
    pub receive: Receiver<Result<NtMessage>>,
    pub send: Sender<NtMessage>,
    pub topics: Mutex<Topics>,
    pub time_offset: AtomicI64,

    pub subuid: AtomicU32,
    pub pubuid: AtomicU32,

    pub last_time_update: AtomicU64,
}

impl InnerNetworkTableClient {
    pub async fn new<B: Backend>(host: &str, name: &str) -> Result<(Self, B::Output)> {
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

                last_time_update: AtomicU64::new(get_time()),
            },
            out,
        ))
    }

    pub fn get_server_time(&self) -> u64 {
        let offset = self.time_offset.load(std::sync::atomic::Ordering::Relaxed);
        (get_time() as i64 + offset) as u64
    }

    pub async fn main_loop<T: Timer>(&self) -> Result<()> {
        select! {
            res = self.time_loop::<T>().fuse() => {
                return res;
            }
            res = self.recv_loop().fuse() => {
                return res;
            }
        }
    }

    pub async fn time_loop<T: Timer>(&self) -> Result<()> {
        loop {
            if get_time()
                - self
                    .last_time_update
                    .load(std::sync::atomic::Ordering::Relaxed)
                > Duration::from_secs(3).as_micros() as u64
            {
                self.send
                    .send(NtMessage::Reconnect(None))
                    .map_err(|_| Error::Send)?;
            }

            T::time(Duration::from_secs(2)).await;

            self.start_sync_time()?;
        }
    }

    pub async fn recv_loop(&self) -> Result<()> {
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

                        if let Some(sender) = topics.topics.get(&name) {
                            if sender.send(SubscriberUpdate::Type(data_type)).is_err()
                                || sender
                                    .send(SubscriberUpdate::Properties(properties))
                                    .is_err()
                            {
                                topics.topics.remove(&name);
                            } else {
                                topics.topic_ids.insert(id, name);
                            }
                        } else {
                            let (send, receive) = unbounded();

                            topics.topics.insert(name.clone(), send.clone());
                            topics.topic_ids.insert(id, name.clone());

                            let Some((_, sender)) = topics
                                .prefix_channels
                                .iter()
                                .find(|(prefix, _)| name.starts_with(prefix.as_str()))
                            else {
                                continue;
                            };

                            send.send(SubscriberUpdate::Type(data_type)).unwrap();
                            send.send(SubscriberUpdate::Properties(properties)).unwrap();

                            sender.send((name, receive)).map_err(|_| Error::Send)?;
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
                        self.last_time_update
                            .store(get_time(), std::sync::atomic::Ordering::Relaxed);
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
                NtMessage::Reconnect(backlog) => {
                    let backlog = backlog.unwrap();

                    self.last_time_update
                        .store(get_time(), std::sync::atomic::Ordering::Relaxed);

                    self.send
                        .send(NtMessage::Binary(BinaryMessage {
                            id: -1,
                            timestamp: 0,
                            data: BinaryData::Int(get_time() as i64),
                        }))
                        .map_err(|_| Error::Send)?;

                    let mut msg = self.receive.recv_async().await??;

                    while !matches!(&msg, NtMessage::Binary(msg) if msg.id == -1) {
                        msg = self.receive.recv_async().await??;
                    }

                    let NtMessage::Binary(msg) = msg else {
                        unreachable!();
                    };

                    let BinaryData::Int(time) = msg.data else {
                        return Err(Error::Type);
                    };

                    let server_time = (get_time() as i64 - time) / 2 + msg.timestamp as i64;
                    let offset = server_time - get_time() as i64;

                    self.time_offset
                        .store(offset, std::sync::atomic::Ordering::Relaxed);

                    let mut topics = self.topics.lock().unwrap();

                    topics.topic_ids.clear();

                    for (subuid, data) in topics.subscribers.iter() {
                        self.send
                            .send(NtMessage::Text(TextMessage::Subscribe {
                                topics: vec![data.name.clone()],
                                subuid: *subuid,
                                options: data.options.clone(),
                            }))
                            .map_err(|_| Error::Send)?;
                    }

                    for (pubuid, data) in topics.publishers.iter() {
                        self.send
                            .send(NtMessage::Text(TextMessage::Publish {
                                name: data.name.to_owned(),
                                pubuid: *pubuid,
                                data_type: data.data_type.to_owned(),
                                properties: Default::default(),
                            }))
                            .map_err(|_| Error::Send)?;
                    }

                    for message in backlog {
                        match message {
                            NtMessage::Text(message) => match message {
                                TextMessage::SetProperties { name, update } => self
                                    .send
                                    .send(NtMessage::Text(TextMessage::SetProperties {
                                        name,
                                        update,
                                    }))
                                    .map_err(|_| Error::Send)?,
                                TextMessage::Subscribe {
                                    mut topics,
                                    subuid,
                                    options,
                                } => {
                                    self.topics.lock().unwrap().topic_ids.values().for_each(
                                        |name| {
                                            topics
                                                .iter()
                                                .position(|topic| name == topic)
                                                .map(|idx| topics.remove(idx));
                                        },
                                    );
                                    if topics.len() != 0 {
                                        self.send
                                            .send(NtMessage::Text(TextMessage::Subscribe {
                                                topics,
                                                subuid,
                                                options,
                                            }))
                                            .map_err(|_| Error::Send)?;
                                    }
                                }
                                TextMessage::Unsubscribe { subuid } => self
                                    .send
                                    .send(NtMessage::Text(TextMessage::Unsubscribe { subuid }))
                                    .map_err(|_| Error::Send)?,
                                _ => {}
                            },
                            NtMessage::Binary(mut message) => {
                                message.timestamp = self.get_server_time();

                                self.send
                                    .send(NtMessage::Binary(message))
                                    .map_err(|_| Error::Send)?;
                            }
                            NtMessage::Reconnect(_) => {}
                        }
                    }
                }
            }
        }
    }

    pub fn start_sync_time(&self) -> Result<()> {
        self.send
            .send(NtMessage::Binary(BinaryMessage {
                id: -1,
                timestamp: 0,
                data: BinaryData::Int(get_time() as i64),
            }))
            .map_err(|_| Error::Send)?;

        Ok(())
    }

    pub fn subscribe(&self, topics: Vec<String>, options: SubscriptionOptions) -> Result<u32> {
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

    pub fn unsubscribe(&self, id: u32) -> Result<()> {
        self.send
            .send(NtMessage::Text(TextMessage::Unsubscribe { subuid: id }))
            .map_err(|_| Error::Send)?;

        Ok(())
    }

    pub fn publish(&self, name: String, data_type: String, properties: Properties) -> Result<u32> {
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

    pub fn unpublish(&self, id: u32) -> Result<()> {
        self.send
            .send(NtMessage::Text(TextMessage::Unpublish { pubuid: id }))
            .map_err(|_| Error::Send)?;

        Ok(())
    }

    pub fn new_subuid(&self) -> u32 {
        self.subuid
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn new_pubuid(&self) -> u32 {
        self.pubuid
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }
}
