use std::{marker::PhantomData, sync::Arc};

use crate::{
    inner::InnerNetworkTableClient,
    types::{payload::Payload, BinaryMessage, Properties, TextMessage},
    Error, NtMessage, Result,
};

pub struct Publisher<P: Payload> {
    pub(crate) name: String,
    pub(crate) id: u32,
    pub(crate) client: Arc<InnerNetworkTableClient>,
    pub(crate) phantom: PhantomData<P>,
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

        self.client
            .topics
            .lock()
            .unwrap()
            .publishers
            .remove(&self.id);
    }
}
