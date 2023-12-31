use core::panic;
use std::{io::Cursor, str::FromStr, usize};

use http::Uri;
use nt::types::{BinaryMessage, TextMessage};
use tungstenite::{
    client::connect,
    handshake::client::{generate_key, Request},
};

fn main() {
    let uri = Uri::from_str("ws://localhost:5810/nt/rust").unwrap();
    let req = Request::builder()
        .method("GET")
        .header("Host", uri.host().unwrap())
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", generate_key())
        .header("Sec-WebSocket-Protocol", "networktables.first.wpi.edu")
        .uri(uri)
        .body(())
        .unwrap();

    let (mut connection, _res) = connect(req).unwrap();

    let messages = vec![TextMessage::Subscribe {
        topics: vec!["".to_string()],
        subuid: 0,
        options: nt::types::SubscriptionOptions {
            periodic: None,
            all: None,
            topicsonly: None,
            prefix: Some(true),
        },
    }];

    connection
        .send(tungstenite::Message::Text(
            serde_json::to_string(&messages).unwrap(),
        ))
        .unwrap();

    loop {
        let next_message = connection.read().unwrap();

        match next_message {
            tungstenite::Message::Text(text) => {
                let messages: Vec<TextMessage> = serde_json::from_str(&text).unwrap();

                dbg!(messages);
            }
            tungstenite::Message::Binary(buf) => {
                let mut buf = Cursor::new(&buf);

                while (buf.position() as usize) < buf.get_ref().len() {
                    let message = BinaryMessage::from_reader(&mut buf).unwrap();
                    dbg!(message);
                }
            }
            _ => panic!("Unsupported frame"),
        }
    }
}
