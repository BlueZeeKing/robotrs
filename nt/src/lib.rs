use std::{marker::PhantomData, sync::Arc, time::Duration};

use flume::{unbounded, Receiver, RecvError, Sender};
use futures::Future;
use inner::InnerNetworkTableClient;
use publish::Publisher;
use subscribe::Subscriber;
use thiserror::Error;
use types::{payload::Payload, BinaryData, BinaryMessage, Properties, TextMessage};

pub mod backends;
pub(crate) mod inner;
pub mod publish;
pub mod subscribe;
pub mod time;
pub mod types;

/// Any type of message that could be sent or recieved from a websocket
#[derive(Debug)]
pub enum NtMessage {
    Text(TextMessage),
    Binary(BinaryMessage),
    Reconnect(Option<Vec<NtMessage>>),
}

pub(crate) enum SubscriberUpdate {
    Properties(Properties),
    Data(BinaryData),
    Type(String),
    Id(u32),
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

        self.inner
            .topics
            .lock()
            .unwrap()
            .publishers
            .push((name.clone(), id, P::name().to_owned()));

        Ok(Publisher {
            name,
            id,
            client: self.inner.clone(),
            phantom: PhantomData,
        })
    }
}
