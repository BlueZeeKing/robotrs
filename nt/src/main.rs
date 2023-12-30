use std::str::FromStr;

use http::Uri;
use nt::types::TextMessage;
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

    let (mut connection, res) = connect(req).unwrap();

    dbg!(res);

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

    println!("{}", serde_json::to_string_pretty(&messages).unwrap());

    connection
        .send(tungstenite::Message::Text(
            serde_json::to_string(&messages).unwrap(),
        ))
        .unwrap();

    loop {
        dbg!(connection.read().unwrap());
    }
}
