use std::{marker::PhantomData, sync::Arc};

use flume::Receiver;

use crate::{
    inner::InnerNetworkTableClient,
    types::{payload::Payload, Properties},
    Error, Result, SubscriberUpdate,
};

pub struct Subscriber<P: Payload> {
    pub(crate) name: String,
    pub(crate) properties: Option<Properties>,
    pub(crate) input: Receiver<SubscriberUpdate>,
    pub(crate) id: u32,
    pub(crate) client: Arc<InnerNetworkTableClient>,
    pub(crate) phantom: PhantomData<P>,
}

impl<P: Payload> Subscriber<P> {
    fn consume_updates(&mut self) -> Result<Option<P>> {
        let mut data = None;
        for update in self.input.drain() {
            match update {
                SubscriberUpdate::Properties(props) => {
                    if self.properties.is_none() {
                        self.properties = Some(Default::default());
                    }

                    self.properties.as_mut().unwrap().update(props);
                }
                SubscriberUpdate::Data(bin_data) => {
                    data = Some(P::parse(bin_data).map_err(|_| Error::Type)?);
                }
                SubscriberUpdate::Type(val) => {
                    if &val != P::name() {
                        return Err(Error::Type);
                    }
                }
                SubscriberUpdate::Id(id) => self.id = id,
            }
        }

        Ok(data)
    }

    /// Wait for a new payload value to become avaliable
    pub async fn get(&mut self) -> Result<P> {
        if !self.input.is_empty() {
            if let Some(val) = self.consume_updates()? {
                return Ok(val);
            }
        }

        loop {
            let val = self.input.recv_async().await?;

            match val {
                SubscriberUpdate::Properties(props) => {
                    if self.properties.is_none() {
                        self.properties = Some(Default::default());
                    }

                    self.properties.as_mut().unwrap().update(props);
                }
                SubscriberUpdate::Data(val) => {
                    break P::parse(val).map_err(|_| Error::Type);
                }
                SubscriberUpdate::Type(ty) => {
                    if &ty != P::name() {
                        return Err(Error::Type);
                    }
                }
                SubscriberUpdate::Id(id) => self.id = id,
            }
        }
    }

    pub fn properties(&self) -> Option<&Properties> {
        self.properties.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl<P: Payload> Drop for Subscriber<P> {
    fn drop(&mut self) {
        let _ = self.client.unsubscribe(self.id);
    }
}
