use serde::de::DeserializeOwned;

use crate::types::BinaryData;

pub trait Payload: Sized {
    fn name() -> &'static str;
    fn parse(data: BinaryData) -> Result<Self, ()>;
}

impl Payload for bool {
    fn name() -> &'static str {
        "boolean"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Boolean(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl Payload for f64 {
    fn name() -> &'static str {
        "double"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Double(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl Payload for f32 {
    fn name() -> &'static str {
        "float"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Float(val) => Ok(val),
            _ => Err(()),
        }
    }
}

macro_rules! payload_num {
    ($value:ident) => {
        impl Payload for $value {
            fn name() -> &'static str {
                "int"
            }

            fn parse(data: BinaryData) -> Result<Self, ()> {
                match data {
                    BinaryData::Int(val) => Ok(val as $value),
                    _ => Err(()),
                }
            }
        }
    };
}

payload_num!(i128);
payload_num!(i64);
payload_num!(i32);
payload_num!(i16);
payload_num!(i8);
payload_num!(u128);
payload_num!(u64);
payload_num!(u32);
payload_num!(u16);
payload_num!(u8);

impl Payload for String {
    fn name() -> &'static str {
        "string"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Str(val) => Ok(val),
            _ => Err(()),
        }
    }
}

pub struct Json<D: DeserializeOwned>(D);

impl<D: DeserializeOwned> Payload for Json<D> {
    fn name() -> &'static str {
        "string"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Str(val) => Ok(Json(serde_json::from_str(&val).map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

impl Payload for Vec<u8> {
    fn name() -> &'static str {
        "raw"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Bin(val) => Ok(val),
            _ => Err(()),
        }
    }
}

pub struct MsgPack(Vec<u8>);

impl Payload for MsgPack {
    fn name() -> &'static str {
        "msgpack"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Bin(val) => Ok(MsgPack(val)),
            _ => Err(()),
        }
    }
}

pub struct Rpc(Vec<u8>);

impl Payload for Rpc {
    fn name() -> &'static str {
        "rpc"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Bin(val) => Ok(Rpc(val)),
            _ => Err(()),
        }
    }
}

pub struct ProtoBuf(Vec<u8>);

impl Payload for ProtoBuf {
    fn name() -> &'static str {
        "protobuf"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Bin(val) => Ok(ProtoBuf(val)),
            _ => Err(()),
        }
    }
}

impl Payload for Vec<bool> {
    fn name() -> &'static str {
        "boolean[]"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::BoolArray(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl Payload for Vec<f64> {
    fn name() -> &'static str {
        "double[]"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::DoubleArray(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl Payload for Vec<f32> {
    fn name() -> &'static str {
        "float[]"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::FloatArray(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl Payload for Vec<i64> {
    fn name() -> &'static str {
        "int[]"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::IntArray(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl Payload for Vec<String> {
    fn name() -> &'static str {
        "string[]"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::StringArray(val) => Ok(val),
            _ => Err(()),
        }
    }
}
