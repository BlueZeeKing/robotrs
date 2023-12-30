use std::{sync::OnceLock, time::Instant};

static START_TIME: OnceLock<Instant> = OnceLock::new();

pub fn get_time() -> u64 {
    // TODO: Use FPGA when on robot
    START_TIME.get_or_init(Instant::now).elapsed().as_micros() as u64
}

pub fn init_time() {
    START_TIME.get_or_init(Instant::now);
}
