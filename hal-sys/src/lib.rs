#[allow(warnings)]
pub mod bindings { include!(concat!(env!("OUT_DIR"), "/bindings.rs")); }

pub fn accel() -> f64 {
    unsafe {
        bindings::HAL_GetAccelerometerZ()
    }
}
