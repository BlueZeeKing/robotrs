use std::mem::MaybeUninit;

#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn test() {
    let mut ahrs = MaybeUninit::uninit();

    unsafe {
        bindings::AHRS_AHRS(ahrs.as_mut_ptr(), hal_sys::HAL_SPIPort_HAL_SPI_kMXP as u32);
    }

    // let yaw = unsafe { bindings::AHRS_GetYaw(ahrs.as_mut_ptr()) };
}
