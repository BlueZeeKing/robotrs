use std::time::Duration;

use crate::{NT_PubSubOptions, NT_Publisher, Publisher};

/// NetworkTables publish/subscribe options.
#[derive(Default, Clone)]
pub struct PubSubOptions {
    poll_storage: Option<u32>,
    periodic: Option<Duration>,
    exclude_publisher: Option<NT_Publisher>,
    send_all: Option<bool>,
    topics_only: Option<bool>,
    prefix_match: Option<bool>,
    keep_duplicates: Option<bool>,
    disable_remote: Option<bool>,
    disable_local: Option<bool>,
    exclude_self: Option<bool>,
    hidden: Option<bool>,
}

impl PubSubOptions {
    /// Polling storage size for a subscription. Specifies the maximum number of\n updates NetworkTables should store between calls to the subscriber's\n ReadQueue() function. If zero, defaults to 1 if sendAll is false, 20 if\n sendAll is true.
    pub fn poll_storage(self, poll_storage: u32) -> Self {
        Self {
            poll_storage: Some(poll_storage),
            ..self
        }
    }

    /// How frequently changes will be sent over the network, in seconds.\n NetworkTables may send more frequently than this (e.g. use a combined\n minimum period for all values) or apply a restricted range to this value.\n The default is 100 ms.
    pub fn periodic(self, periodic: Duration) -> Self {
        Self {
            periodic: Some(periodic),
            ..self
        }
    }

    /// For subscriptions, if non-zero, value updates for ReadQueue() are not\n queued for this publisher.
    pub fn exclude_publisher<T>(self, publisher: Publisher<T>) -> Self {
        Self {
            exclude_publisher: Some(publisher.handle),
            ..self
        }
    }

    /// Send all value changes over the network.
    pub fn send_all(self, send_all: bool) -> Self {
        Self {
            send_all: Some(send_all),
            ..self
        }
    }

    /// For subscriptions, don't ask for value changes (only topic announcements).
    pub fn topics_only(self, topics_only: bool) -> Self {
        Self {
            topics_only: Some(topics_only),
            ..self
        }
    }

    /// Perform prefix match on subscriber topic names. Is ignored/overridden by\n Subscribe() functions; only present in struct for the purposes of getting\n information about subscriptions.
    pub fn prefix_match(self, prefix_match: bool) -> Self {
        Self {
            prefix_match: Some(prefix_match),
            ..self
        }
    }

    /// Preserve duplicate value changes (rather than ignoring them).
    pub fn keep_duplicates(self, keep_duplicates: bool) -> Self {
        Self {
            keep_duplicates: Some(keep_duplicates),
            ..self
        }
    }

    /// For subscriptions, if remote value updates should not be queued for\n ReadQueue(). See also disableLocal.
    pub fn disable_remote(self, disable_remote: bool) -> Self {
        Self {
            disable_remote: Some(disable_remote),
            ..self
        }
    }

    /// For subscriptions, if local value updates should not be queued for\n ReadQueue(). See also disableRemote.
    pub fn disable_local(self, disable_local: bool) -> Self {
        Self {
            disable_local: Some(disable_local),
            ..self
        }
    }

    /// For entries, don't queue (for ReadQueue) value updates for the entry's\n internal publisher.
    pub fn exclude_self(self, exclude_self: bool) -> Self {
        Self {
            exclude_self: Some(exclude_self),
            ..self
        }
    }

    /// For subscriptions, don't share the existence of the subscription with the\n network. Note this means updates will not be received from the network\n unless another subscription overlaps with this one, and the subscription\n will not appear in metatopics.
    pub fn hidden(self, hidden: bool) -> Self {
        Self {
            hidden: Some(hidden),
            ..self
        }
    }

    pub(crate) fn build(self) -> NT_PubSubOptions {
        NT_PubSubOptions {
            structSize: size_of::<NT_PubSubOptions>() as u32,
            pollStorage: self.poll_storage.unwrap_or_else(|| {
                if self.send_all.unwrap_or(false) {
                    20
                } else {
                    1
                }
            }),
            periodic: self
                .periodic
                .unwrap_or_else(|| Duration::from_millis(100))
                .as_secs_f64(),
            excludePublisher: self.exclude_publisher.unwrap_or(0),
            sendAll: if self.send_all.unwrap_or(false) { 1 } else { 0 },
            topicsOnly: if self.topics_only.unwrap_or(false) {
                1
            } else {
                0
            },
            prefixMatch: if self.prefix_match.unwrap_or(false) {
                1
            } else {
                0
            },
            keepDuplicates: if self.keep_duplicates.unwrap_or(false) {
                1
            } else {
                0
            },
            disableRemote: if self.disable_remote.unwrap_or(false) {
                1
            } else {
                0
            },
            disableLocal: if self.disable_local.unwrap_or(false) {
                1
            } else {
                0
            },
            excludeSelf: if self.exclude_self.unwrap_or(false) {
                1
            } else {
                0
            },
            hidden: if self.hidden.unwrap_or(false) { 1 } else { 0 },
        }
    }
}
