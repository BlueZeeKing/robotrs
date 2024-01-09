use core::panic;
use flume::RecvError;
use futures::{sink::SinkExt, stream::StreamExt, FutureExt};
use http::{header::SEC_WEBSOCKET_PROTOCOL, uri::InvalidUri, Request};
use std::{io::Cursor, str::FromStr, time::Duration};
use tokio::{select, task::JoinHandle};
use tokio_tungstenite::connect_async;
use tungstenite::{handshake::client::generate_key, Message};

use http::Uri;

use crate::{
    types::{BinaryMessage, BinaryMessageError, TextMessage},
    Backend, Error, Result, Timer,
};

#[derive(Debug, Error)]
pub enum TokioError {
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

pub struct TokioBackend {}

impl Backend for TokioBackend {
    type Output = JoinHandle<()>;
    type Error = TokioError;

    fn create(
        host: &str,
        name: &str,
        send: flume::Sender<Result<crate::NtMessage>>,
        receive: flume::Receiver<crate::NtMessage>,
    ) -> Result<Self::Output, TokioError> {
        let uri = Uri::from_str(&format!("ws://{host}:5810/nt/{name}"))?;

        let send2 = send.clone();

        Ok(tokio::spawn(async move {
            let req = Request::builder()
                .method("GET")
                .header("Host", uri.host().unwrap())
                .header("Connection", "Upgrade")
                .header("Upgrade", "websocket")
                .header("Sec-WebSocket-Version", "13")
                .header("Sec-WebSocket-Key", generate_key())
                .header("Sec-WebSocket-Protocol", "networktables.first.wpi.edu")
                .uri(uri)
                .body(())?;

            let (mut connection, res) = connect_async(req.clone()).await?;

            if res
                .headers()
                .get(SEC_WEBSOCKET_PROTOCOL)
                .ok_or(TokioError::UnsupportedServer)?
                != "networktables.first.wpi.edu"
            {
                return Err(TokioError::UnsupportedServer);
            }


            loop {
                select! {
                    message = receive.recv_async() => {
                        let message = message?;

                        match message {
                            crate::NtMessage::Text(msg) => connection.send(Message::Text(serde_json::to_string(&[msg])?)).await?,
                            crate::NtMessage::Binary(msg) => {
                                let mut buf = Vec::new();
                                msg.to_writer(&mut buf)?;
                                connection.send(Message::Binary(buf)).await?
                            }
                            crate::NtMessage::Reconnect(_) => {
                                loop {
                                    if let Ok((mut new_con, _)) = connect_async(req.clone()).await {
                                        std::mem::swap(&mut new_con, &mut connection);

                                        send.send(Ok(crate::NtMessage::Reconnect(Some(receive.drain().collect::<Vec<_>>())))).map_err(|_| TokioError::Send)?;

                                        break;
                                    } else {
                                        tokio::time::sleep(Duration::from_secs(1)).await;
                                    }
                                }
                            },
                        }
                    }
                    message = connection.next() => {
                        if matches!(message, Some(Err(_))) || message.is_none() {
                            loop {
                                if let Ok((mut new_con, _)) = connect_async(req.clone()).await {
                                    std::mem::swap(&mut new_con, &mut connection);

                                    send.send(Ok(crate::NtMessage::Reconnect(Some(receive.drain().collect::<Vec<_>>())))).map_err(|_| TokioError::Send)?;

                                    receive.drain().for_each(|_| {});

                                    break;
                                } else {
                                    tokio::time::sleep(Duration::from_secs(1)).await;
                                }
                            }

                            continue;
                        }

                        let message = message.unwrap()?;

                        match message {
                            Message::Text(msg) => {
                                let msgs = serde_json::from_str::<Vec<TextMessage>>(&msg)?;
                                for msg in msgs {
                                    send.send(Ok(crate::NtMessage::Text(msg))).map_err(|_| TokioError::Send)?;
                                }
                            }
                            Message::Binary(msg) => {
                                let mut cursor = Cursor::new(msg);

                                while (cursor.position() as usize) < cursor.get_ref().len() {
                                    send.send(Ok(crate::NtMessage::Binary(BinaryMessage::from_reader(&mut cursor)?))).map_err(|_| TokioError::Send)?;
                                }
                            }
                            _ => return <Result<(), TokioError>>::Err(TokioError::UnknownFrame),
                        }
                    }
                }
            }
        }.map(move |out| {
            if let Err(err) = out {
                let _res = send2.send(Err(Error::Other(Box::new(err))));
            }
        })))
    }
}

impl Timer for TokioBackend {
    async fn time(duration: std::time::Duration) {
        tokio::time::sleep(duration).await;
    }
}
