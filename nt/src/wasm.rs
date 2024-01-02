use std::io::Cursor;

use thiserror::Error;
use wasm_bindgen_futures::{
    js_sys::{ArrayBuffer, JsString, Uint8Array},
    spawn_local,
    wasm_bindgen::{closure::Closure, JsCast},
};
use web_sys::{wasm_bindgen::JsValue, MessageEvent, WebSocket};

use crate::{
    types::{BinaryMessage, TextMessage},
    Backend, Timer,
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

impl Backend for WasmBackend {
    type Output = ();
    type Error = Error;

    fn create(
        host: &str,
        name: &str,
        send: flume::Sender<crate::Result<crate::NtMessage>>,
        receive: flume::Receiver<crate::NtMessage>,
    ) -> std::result::Result<Self::Output, Self::Error> {
        let ws = WebSocket::new_with_str(
            &format!("ws://{host}:5810/nt/{name}"),
            "networktables.first.wpi.edu",
        )?;

        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(buf) = e.data().dyn_into::<ArrayBuffer>() {
                let array = Uint8Array::new(&buf);
                let mut buf = Vec::with_capacity(array.length() as usize);
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
        });

        ws.set_onmessage(Some(callback.as_ref().unchecked_ref()));

        callback.forget();

        spawn_local(async move {
            loop {
                match receive.recv_async().await.unwrap() {
                    crate::NtMessage::Text(msg) => ws
                        .send_with_str(&serde_json::to_string(&[msg]).unwrap())
                        .unwrap(),
                    crate::NtMessage::Binary(msg) => {
                        let mut buf = Vec::new();
                        msg.to_writer(&mut buf).unwrap();
                        ws.send_with_u8_array(&buf).unwrap();
                    }
                }
            }
        });

        Ok(())
    }
}

impl Timer for WasmBackend {
    async fn time(duration: std::time::Duration) {
        wasm_timer::Delay::new(duration).await;
    }
}
