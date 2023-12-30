use serde::{Deserialize, Serialize};

fn should_skip(val: &MissingOrNull<bool>) -> bool {
    *val == MissingOrNull::Missing
}

fn skip_none<T>(val: &Option<T>) -> bool {
    val.is_none()
}

/// Each published topic may also have properties associated to it. Properties are represented in
/// the protocol as JSON and thus property values may be any JSON type. Property keys must be
/// strings. The following properties have a defined meaning in this spec. Servers shall support
/// arbitrary properties being set outside of this set. Clients shall ignore properties they do not
/// recognize. Properties are initially set on publish and may be changed (by any client) using
/// [TextMessage::SetProperties]
#[derive(Serialize, Deserialize)]
pub struct Properties {
    /// If true, the last set value will be periodically saved to persistent storage on the server
    /// and be restored during server startup. Topics with this property set to true will not be
    /// deleted by the server when the last publisher stops publishing.
    #[serde(
        with = "missing_or_null_impls",
        default,
        skip_serializing_if = "should_skip"
    )]
    pub persistent: MissingOrNull<bool>,
    /// Topics with this property set to true will not be deleted by the server when the last
    /// publisher stops publishing.
    #[serde(
        with = "missing_or_null_impls",
        default,
        skip_serializing_if = "should_skip"
    )]
    pub retained: MissingOrNull<bool>,
    /// If false, the server and clients will not store the value of the topic. This means that
    /// only value updates will be available for the topic.
    #[serde(
        with = "missing_or_null_impls",
        default,
        skip_serializing_if = "should_skip"
    )]
    pub cached: MissingOrNull<bool>,
}

mod missing_or_null_impls {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use super::MissingOrNull;

    pub fn serialize<S: Serializer>(
        value: &MissingOrNull<bool>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        <Option<bool>>::from(value.to_owned()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<MissingOrNull<bool>, D::Error> {
        <Option<bool>>::deserialize(deserializer).map(|option| option.into())
    }
}

/// Each subscription may have options set. The following options have a defined meaning in this
/// spec. Servers shall preserve arbitrary options, as servers and clients may support arbitrary
/// options outside of this set. Options are set using Subscribe Message ([TextMessage::Subscribe])
/// and cannot be changed.
#[derive(Serialize, Deserialize)]
pub struct SubscriptionOptions {
    /// How frequently the server should send changes. The server may send more frequently than
    /// this (e.g. use a combined minimum period for all values) or apply a restricted range to
    /// this value. The default if unspecified is 100 ms (same as NT 3.0).
    #[serde(skip_serializing_if = "skip_none", default)]
    pub periodic: Option<u32>,

    /// If true, the server should send all value changes over the wire. If false, only the most
    /// recent value is sent (same as NT 3.0 behavior). If not specified, defaults to false.
    #[serde(skip_serializing_if = "skip_none", default)]
    pub all: Option<bool>,

    /// If true, the server should not send any value changes over the wire regardless of other
    /// options. This is useful for only getting topic announcements. If false, value changes are
    /// sent in accordance with other options. If not specified, defaults to false.
    #[serde(skip_serializing_if = "skip_none", default)]
    pub topicsonly: Option<bool>,

    /// If true, any topic starting with the name in the subscription topics list is subscribed to,
    /// not just exact matches. If not specified, defaults to false.
    #[serde(skip_serializing_if = "skip_none", default)]
    pub prefix: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum TextMessage {
    /// Sent from a client to the server to indicate the client wants to start publishing values at
    /// the given topic. The server shall respond with a Topic Announcement Message
    /// ([TextMessage::Announce]), even if the topic was previously announced. The client can start
    /// publishing data values via MessagePack messages immediately after sending this message, but
    /// the messages will be ignored by the server if the publisher data type does not match the
    /// topic data type.
    #[serde(rename = "publish")]
    Publish {
        /// The topic name being published
        name: String,

        /// A client-generated unique identifier for this publisher. Use the same UID later to
        /// unpublish. This is also the identifier that the client will use in MessagePack messages
        /// for this topic.
        pubuid: i32,

        /// The requested data type (as a string).
        ///
        /// If the topic is newly created (e.g. there are no other publishers) this sets the value
        /// type. If the topic was previously published, this is ignored. The
        /// [TextMessage::Announce] message contains the actual topic value type that the client
        /// shall use when publishing values.
        ///
        /// Implementations should indicate an error if the user tries to publish an incompatible
        /// type to that already set for the topic.
        #[serde(rename = "type")]
        data_type: String, // TODO: Make real type

        /// Initial topic properties.
        ///
        /// If the topic is newly created (e.g. there are no other publishers) this sets the topic
        /// properties. If the topic was previously published, this is ignored. The
        /// [TextMessage::Announce] message contains the actual topic properties. Clients can use
        /// the [TextMessage::SetProperties] message to change properties after topic creation.
        properties: Properties,
    },

    /// Sent from a client to the server to indicate the client wants to stop publishing values for
    /// the given topic and publisher. The client should stop publishing data value updates via
    /// binary MessagePack messages for this publisher prior to sending this message.
    ///
    /// When there are no remaining publishers for a non-persistent topic, the server shall delete
    /// the topic and send a Topic Removed Message ([TextMessage::Unannounce]) to all clients who
    /// have been sent a previous Topic Announcement Message ([TextMessage::Announce]) for the
    /// topic.
    #[serde(rename = "unpublish")]
    Unpublish {
        /// The same unique identifier passed to the [TextMessage::Publish] message
        pubuid: i32,
    },

    /// Sent from a client to the server to change properties (see [Properties]) for a given topic.
    /// The server will send a corresponding Properties Update Message ([TextMessage::Properties])
    /// to all subscribers to the topic (if the topic is published). This message shall be ignored
    /// by the server if the topic is not published.
    #[serde(rename = "setproperties")]
    SetProperties { name: String, update: Properties },

    /// Sent from a client to the server to indicate the client wants to subscribe to value changes
    /// for the specified topics / groups of topics. The server shall send MessagePack messages
    /// containing the current values for any existing cached topics upon receipt, and continue
    /// sending MessagePack messages for future value changes. If a topic does not yet exist, no
    /// message is sent until it is created (via a publish), at which point a Topic Announcement
    /// Message ([TextMessage::Announce]) will be sent and MessagePack messages will automatically
    /// follow as they are published.
    ///
    /// Subscriptions may overlap; only one MessagePack message is sent per value change regardless
    /// of the number of subscriptions. Sending a subscribe message with the same subscription UID
    /// as a previous subscribe message results in updating the subscription (replacing the array
    /// of identifiers and updating any specified options).
    #[serde(rename = "subscribe")]
    Subscribe {
        /// One or more topic names or prefixes (if the prefix option is true) to start receiving
        /// messages for.
        topics: Vec<String>,

        /// A client-generated unique identifier for this subscription. Use the same UID later to
        /// unsubscribe.
        subuid: i32,

        /// [SubscriptionOptions]
        options: SubscriptionOptions,
    },

    /// Sent from a client to the server to indicate the client wants to stop subscribing to
    /// messages for the given subscription.
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        /// The same unique identifier passed to the [TextMessage::Subscribe] message
        subuid: i32,
    },

    /// The server shall send this message for each of the following conditions:
    /// - To all clients subscribed to a matching prefix when a topic is created
    /// - To a client in response to an Publish Request Message ([TextMessage::Publish]) from that client
    #[serde(rename = "announce")]
    Announce {
        name: String,

        /// The identifier that the server will use in MessagePack messages for this topic
        id: i32,

        /// The data type for the topic (as a string)
        data_type: String,

        /// If this message was sent in response to a [TextMessage::Publish] message, the Publisher UID provided
        /// in that message. Otherwise absent.
        pubuid: Option<i32>,

        /// Topic [Properties]
        properties: Properties,
    },

    /// The server shall send this message when a previously announced (via a Topic Announcement
    /// Message ([TextMessage::Announce])) topic is deleted.
    #[serde(rename = "unannounce")]
    Unannounce {
        name: String,

        /// The identifier that the server was using for value updates
        id: i32,
    },

    /// The server shall send this message when a previously announced (via a Topic Announcement
    /// Message ([TextMessage::Announce])) topic has its properties changed (via Set Properties Message
    /// ([TextMessage::SetProperties])).
    #[serde(rename = "properties")]
    Properties {
        name: String,

        /// True if this message is in response to a [TextMessage::SetProperties] message from the
        /// same client. Otherwise absent.
        ack: bool,

        /// The client shall handle the update value as follows. If a property is not included in
        /// the update map, its value is not changed. If a property is provided in the update map
        /// with a value of null, the property is deleted.
        update: Properties,
    },
}

#[derive(PartialEq, Clone)]
pub enum MissingOrNull<T> {
    Missing,
    Null,
    Value(T),
}

impl<T: Copy> Copy for MissingOrNull<T> {}

impl<T> From<Option<T>> for MissingOrNull<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(val) => MissingOrNull::Value(val),
            None => MissingOrNull::Null,
        }
    }
}

impl<T> From<MissingOrNull<T>> for Option<T> {
    fn from(value: MissingOrNull<T>) -> Option<T> {
        match value {
            MissingOrNull::Missing | MissingOrNull::Null => None,
            MissingOrNull::Value(val) => Some(val),
        }
    }
}

impl<T> Default for MissingOrNull<T> {
    fn default() -> Self {
        Self::Missing
    }
}
