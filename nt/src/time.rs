#[cfg(not(feature = "wasm"))]
static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();

pub fn get_time() -> u64 {
    // TODO: Use FPGA when on robot
    #[cfg(not(feature = "wasm"))]
    {
        START_TIME
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_micros() as u64
    }
    #[cfg(feature = "wasm")]
    {
        let millis = web_sys::window().unwrap().performance().unwrap().now();

        (millis * 1000.0) as u64
    }
}

pub fn init_time() {
    #[cfg(not(feature = "wasm"))]
    START_TIME.get_or_init(std::time::Instant::now);
}
