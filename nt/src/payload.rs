use serde::{de::DeserializeOwned, Serialize};

use crate::types::BinaryData;

pub trait Payload: Sized {
    fn name() -> &'static str;
    fn parse(data: BinaryData) -> Result<Self, ()>;
    fn to_val(self) -> BinaryData;
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

    fn to_val(self) -> BinaryData {
        BinaryData::Boolean(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::Double(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::Float(self)
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

            fn to_val(self) -> BinaryData {
                BinaryData::Int(self as i64)
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

    fn to_val(self) -> BinaryData {
        BinaryData::Str(self)
    }
}

pub struct Json<D: DeserializeOwned + Serialize>(D);

impl<D: DeserializeOwned + Serialize> Payload for Json<D> {
    fn name() -> &'static str {
        "string"
    }

    fn parse(data: BinaryData) -> Result<Self, ()> {
        match data {
            BinaryData::Str(val) => Ok(Json(serde_json::from_str(&val).map_err(|_| ())?)),
            _ => Err(()),
        }
    }

    fn to_val(self) -> BinaryData {
        BinaryData::Str(serde_json::to_string(&self.0).unwrap())
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

    fn to_val(self) -> BinaryData {
        BinaryData::Bin(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::Bin(self.0)
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

    fn to_val(self) -> BinaryData {
        BinaryData::Bin(self.0)
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

    fn to_val(self) -> BinaryData {
        BinaryData::Bin(self.0)
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

    fn to_val(self) -> BinaryData {
        BinaryData::BoolArray(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::DoubleArray(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::FloatArray(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::IntArray(self)
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

    fn to_val(self) -> BinaryData {
        BinaryData::StringArray(self)
    }
}
