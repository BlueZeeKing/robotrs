#![allow(warnings)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const WPI_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/version.txt"));
