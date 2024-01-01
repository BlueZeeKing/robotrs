use core::panic;
use futures::{sink::SinkExt, stream::StreamExt, FutureExt};
use http::{header::SEC_WEBSOCKET_PROTOCOL, Request};
use std::{io::Cursor, str::FromStr};
use tokio::select;
use tokio_tungstenite::connect_async;
use tungstenite::{handshake::client::generate_key, Message};

use http::Uri;

use crate::{types::BinaryMessage, Backend, Error, Result, Timer};

pub struct TokioBackend {}

impl Backend for TokioBackend {
    type Output = ();
    type Error = crate::Error;

    fn create(
        host: &str,
        name: &str,
        send: flume::Sender<Result<crate::NtMessage>>,
        receive: flume::Receiver<crate::NtMessage>,
    ) -> Result<Self::Output> {
        let uri = Uri::from_str(&format!("ws://{host}:5810/nt/{name}"))?;

        let send2 = send.clone();

        tokio::spawn(async move {
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

            let (mut connection, res) = connect_async(req).await?;

            if res
                .headers()
                .get(SEC_WEBSOCKET_PROTOCOL)
                .ok_or(Error::UnsupportedServer)?
                != "networktables.first.wpi.edu"
            {
                return Err(Error::UnsupportedServer);
            }

            loop {
                select! {
                    message = receive.recv_async() => {
                        let message = message?;

                        match message {
                            crate::NtMessage::Text(msg) => connection.send(Message::Text(serde_json::to_string(&msg)?)).await?,
                            crate::NtMessage::Binary(msg) => {
                                let mut buf = Vec::new();
                                msg.to_writer(&mut buf)?;
                                connection.send(Message::Binary(buf)).await?
                            },
                        }
                    }
                    message = connection.next() => {
                        if message.is_none() {
                            return Ok(());
                        }
                        let message = message.unwrap()?;

                        match message {
                            Message::Text(msg) => send.send(Ok(crate::NtMessage::Text(serde_json::from_str(&msg)?))).map_err(|_| Error::Send)?,
                            Message::Binary(msg) => send.send(Ok(crate::NtMessage::Binary(BinaryMessage::from_reader(&mut Cursor::new(msg))?))).map_err(|_| Error::Send)?,
                            _ => return <Result<()>>::Err(Error::UnknownFrame),
                        }
                    }
                }
            }
        }.map(move |out| if let Err(err) = out { send2.send(Err(err)).unwrap() }));

        Ok(())
    }
}

impl Timer for TokioBackend {
    async fn time(duration: std::time::Duration) {
        tokio::time::sleep(duration).await;
    }
}
