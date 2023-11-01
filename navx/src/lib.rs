use std::mem::MaybeUninit;

// #[allow(warnings)]
// mod bindings {
//     include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// }

#[allow(warnings)]
mod bindings;

pub struct NavX(bindings::AHRS);

impl NavX {
    pub fn with_port(port: u32) -> Self {
        Self(unsafe {
            let mut ahrs = MaybeUninit::uninit();
            bindings::AHRS_AHRS(ahrs.as_mut_ptr(), port);
            ahrs.assume_init()
        })
    }

    pub fn new() -> Self {
        Self::with_port(hal_sys::HAL_SPIPort_HAL_SPI_kMXP as u32)
    }

    pub fn get_yaw(&mut self) -> f32 {
        unsafe { bindings::AHRS_GetYaw(&mut self.0) }
    }
}

impl Default for NavX {
    fn default() -> Self {
        Self::new()
    }
}
