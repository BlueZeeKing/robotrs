use linkme::distributed_slice;

pub mod error;
pub mod hid;
pub mod time;

#[distributed_slice]
pub static PERIODIC_CHECKS: [fn()] = [..];
