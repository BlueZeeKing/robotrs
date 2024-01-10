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
    pub(crate) prefix_children: Option<Receiver<(String, Receiver<SubscriberUpdate>)>>,
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
                    if matches!(P::name(), Some(data_type) if data_type != val) {
                        return Err(Error::Type);
                    }
                }
            }
        }

        Ok(data)
    }

    /// Wait for a new payload value to become avaliable
    pub async fn get(&mut self) -> Result<P> {
        dbg!("get");
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
                    if matches!(P::name(), Some(data_type) if data_type != ty) {
                        return Err(Error::Type);
                    }
                }
            }
        }
    }

    pub fn properties(&self) -> Option<&Properties> {
        self.properties.as_ref()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn get_child<PC: Payload>(&self) -> Result<PrefixSubscriber<'_, PC>> {
        let receiver = self
            .prefix_children
            .as_ref()
            .expect("Tried to get children from a topic wihout the prefix option set");

        let (name, channel) = receiver.recv_async().await?;

        Ok(PrefixSubscriber {
            name,
            input: channel,
            properties: None,
            phantom: PhantomData,
        })
    }
}

impl<P: Payload> Drop for Subscriber<P> {
    fn drop(&mut self) {
        let _ = self.client.unsubscribe(self.id);
        self.client
            .topics
            .lock()
            .unwrap()
            .subscribers
            .remove(&self.id);
    }
}

/// A subscriber created as a child of a subscriber
pub struct PrefixSubscriber<'a, P: Payload> {
    name: String,
    input: Receiver<SubscriberUpdate>,
    properties: Option<Properties>,
    phantom: PhantomData<&'a P>,
}

impl<'a, P: Payload> PrefixSubscriber<'a, P> {
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
                    if matches!(P::name(), Some(data_type) if data_type != val) {
                        return Err(Error::Type);
                    }
                }
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
                    if matches!(P::name(), Some(data_type) if data_type != ty) {
                        return Err(Error::Type);
                    }
                }
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
