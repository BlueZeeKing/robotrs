use std::{io::Cursor, sync::Arc};

use event_listener::Event;
use flume::{Receiver, Sender};
use futures::{select, FutureExt};
use thiserror::Error;
use wasm_bindgen_futures::{
    js_sys::{ArrayBuffer, JsString, Uint8Array},
    spawn_local,
    wasm_bindgen::{closure::Closure, JsCast},
};
use web_sys::{wasm_bindgen::JsValue, MessageEvent, WebSocket};

use crate::{
    types::{BinaryMessage, TextMessage},
    Backend, NtMessage, Timer,
};

pub struct WasmBackend {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered a JS error")]
    Js,
}

impl From<JsValue> for Error {
    fn from(_value: JsValue) -> Self {
        Self::Js
    }
}

fn create_msg_callback(send: Sender<crate::Result<NtMessage>>) -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(buf) = e.data().dyn_into::<ArrayBuffer>() {
            let array = Uint8Array::new(&buf);
            let mut buf = vec![0; array.length() as usize];
            array.copy_to(&mut buf);

            let mut reader = Cursor::new(buf);

            while (reader.position() as usize) < reader.get_ref().len() {
                let _ = send.send(Ok(crate::NtMessage::Binary(
                    BinaryMessage::from_reader(&mut reader).unwrap(),
                )));
            }
        } else if let Ok(text) = e.data().dyn_into::<JsString>() {
            let text: String = text.into();
            let msgs = serde_json::from_str::<Vec<TextMessage>>(&text).unwrap();
            for msg in msgs {
                send.send(Ok(crate::NtMessage::Text(msg))).unwrap();
            }
        }
    })
}

fn create_ready_callback(
    receive: Receiver<NtMessage>,
    ws: WebSocket,
    event: Arc<Event>,
) -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(_)>::new(move |_e: MessageEvent| {
        let receive2 = receive.clone();
        let ws2 = ws.clone();
        let event2 = event.clone();

        spawn_local(async move {
            select! {
                _ = main_loop(receive2, ws2).fuse() => {},
                _ = event2.listen().fuse() => {}
            }
        });
    })
}

async fn main_loop(receive: Receiver<NtMessage>, ws: WebSocket) {
    loop {
        let recv = receive.recv_async().await.unwrap();
        match recv {
            crate::NtMessage::Text(msg) => ws
                .send_with_str(&serde_json::to_string(&[msg]).unwrap())
                .unwrap(),
            crate::NtMessage::Binary(msg) => {
                let mut buf = Vec::new();
                msg.to_writer(&mut buf).unwrap();
                ws.send_with_u8_array(&buf).unwrap();
            }
            crate::NtMessage::Reconnect(_) => {
                ws.close().unwrap();
            }
        }
    }
}

fn create_close_callback(
    send: Sender<crate::Result<NtMessage>>,
    receive: Receiver<NtMessage>,
    url: String,
    close_event: Arc<Event>,
) -> Closure<dyn FnMut(MessageEvent)> {
    Closure::<dyn FnMut(_)>::new(move |_e: MessageEvent| {
        close_event.notify(u32::MAX);
        let ws = WebSocket::new_with_str(&url, "networktables.first.wpi.edu").unwrap();

        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let new_close_event = Arc::new(Event::new());

        let msg_callback = create_msg_callback(send.clone());
        let ready_callback =
            create_ready_callback(receive.clone(), ws.clone(), new_close_event.clone());
        let close_callback =
            create_close_callback(send.clone(), receive.clone(), url.clone(), new_close_event);

        ws.set_onmessage(Some(msg_callback.as_ref().unchecked_ref()));
        ws.set_onopen(Some(ready_callback.as_ref().unchecked_ref()));
        ws.set_onclose(Some(close_callback.as_ref().unchecked_ref()));

        msg_callback.forget();
        ready_callback.forget();
        close_callback.forget();
    })
}

impl Backend for WasmBackend {
    type Output = ();
    type Error = Error;

    fn create(
        host: &str,
        name: &str,
        send: flume::Sender<crate::Result<crate::NtMessage>>,
        receive: flume::Receiver<crate::NtMessage>,
    ) -> std::result::Result<Self::Output, Self::Error> {
        let url = format!("ws://{host}:5810/nt/{name}");
        let ws = WebSocket::new_with_str(&url, "networktables.first.wpi.edu")?;

        let close_event = Arc::new(Event::new());

        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let msg_callback = create_msg_callback(send.clone());
        let ready_callback =
            create_ready_callback(receive.clone(), ws.clone(), close_event.clone());
        let close_callback = create_close_callback(send, receive, url, close_event);

        ws.set_onmessage(Some(msg_callback.as_ref().unchecked_ref()));
        ws.set_onopen(Some(ready_callback.as_ref().unchecked_ref()));
        ws.set_onclose(Some(close_callback.as_ref().unchecked_ref()));

        msg_callback.forget();
        ready_callback.forget();
        close_callback.forget();

        Ok(())
    }
}

impl Timer for WasmBackend {
    async fn time(duration: std::time::Duration) {
        let _ = fluvio_wasm_timer::Delay::new(duration).await;
    }
}
