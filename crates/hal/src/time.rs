use core::panic;

use embedded_hal::delay::DelayNs;
use hal_sys::{
    HAL_CleanNotifier, HAL_GetFPGATime, HAL_InitializeNotifier, HAL_UpdateNotifierAlarm,
    HAL_WaitForNotifierAlarm,
};
use robotrs::error::HalError;

pub(crate) struct Delay {
    handle: Option<i32>,
}

impl Delay {
    pub fn new() -> Result<Self, HalError> {
        let mut error = 0;
        let handle = unsafe { HAL_InitializeNotifier(&mut error) };
        if error != 0 {
            return Err(HalError::from_raw(error));
        }

        let raw_notifier = Self {
            handle: Some(handle),
        };

        Ok(raw_notifier)
    }
}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        let Some(handle) = self.handle else {
            panic!("Tried to use failed or closed notifier");
        };

        let microseconds = ns / 1000;

        let mut status = 0;

        let time = unsafe { HAL_GetFPGATime(&mut status) };

        if status != 0 {
            panic!("Could not get time while trying to delay");
        }

        unsafe {
            HAL_UpdateNotifierAlarm(handle, time + microseconds as u64, &mut status);
        }

        if status != 0 {
            panic!("Could not update notifier while trying to delay");
        }

        let new_time = unsafe { HAL_WaitForNotifierAlarm(handle, &mut status) };

        if status != 0 {
            panic!("Could not block while trying to delay");
        }

        if new_time == 0 {
            self.handle = None;
        }
    }
}

impl Drop for Delay {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            let mut error = 0;
            unsafe { HAL_CleanNotifier(handle, &mut error) };
            if error != 0 {
                panic!("Failed to cleanup timer with code: {}", error);
            }
        }
    }
}
