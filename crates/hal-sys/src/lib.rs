#![allow(warnings)]

mod bindings;
pub use bindings::*;

pub const WPI_VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/version.txt"));
