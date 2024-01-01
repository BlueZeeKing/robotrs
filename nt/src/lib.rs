use core::panic;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Mutex},
    time::Duration,
};

use flume::{unbounded, Receiver, RecvError, Sender};
use futures::{select, FutureExt};
use http::uri::InvalidUri;
use thiserror::Error;
use time::get_time;
use types::{BinaryData, BinaryMessage, BinaryMessageError, Properties, TextMessage};

pub mod backend;
pub mod payload;
pub mod time;
pub mod types;

#[derive(Debug)]
pub enum NtMessage {
    Text(TextMessage),
    Binary(BinaryMessage),
}

pub struct NetworkTableClient {
    receive: Receiver<Result<NtMessage>>,
    send: Sender<NtMessage>,
    topics: Mutex<HashMap<u32, Topic>>,
    time_offset: AtomicI64,
}

pub struct Topic {
    name: String,
    data_type: String,
    properties: Properties,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered an http error: {0}")]
    Http(#[from] http::Error),
    #[error("Encountered a websocket error: {0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("Invalid uri error: {0}")]
    Uri(#[from] InvalidUri),
    #[error("Server does not support the nt v4.0 protocol")]
    UnsupportedServer,
    #[error("Error while encoding or decoding a binary message: {0}")]
    BinaryMessage(#[from] BinaryMessageError),
    #[error("Error while encoding or decoding a text message: {0}")]
    TextMessage(#[from] serde_json::Error),
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

pub trait Backend {
    type Output;
    type Error: std::error::Error + 'static + Send;

    fn create(
        host: &str,
        name: &str,
        send: Sender<Result<NtMessage>>,
        receive: Receiver<NtMessage>,
    ) -> std::result::Result<Self::Output, Self::Error>;
}

impl NetworkTableClient {
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
                topics: Mutex::new(HashMap::new()),
                time_offset: AtomicI64::new(offset),
            },
            out,
        ))
    }

    pub fn get_server_time(&self) -> u32 {
        let offset = self.time_offset.load(std::sync::atomic::Ordering::Relaxed);
        (get_time() as i64 + offset) as u32
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
                NtMessage::Text(_) => todo!(),
                NtMessage::Binary(msg) => {
                    if msg.id == -1 {
                        let BinaryData::Int(time) = msg.data else {
                            return Err(Error::Type);
                        };

                        let server_time = (get_time() as i64 - time) / 2 + msg.timestamp as i64;
                        let offset = server_time - get_time() as i64;

                        self.time_offset
                            .fetch_min(offset, std::sync::atomic::Ordering::Relaxed);
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
}

pub trait Timer {
    fn time(duration: Duration) -> impl std::future::Future<Output = ()> + Send;
}
