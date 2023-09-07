use std::time::Duration;

use hal_sys::{
    HAL_CleanNotifier, HAL_GetFPGATime, HAL_InitializeNotifier, HAL_UpdateNotifierAlarm,
    HAL_WaitForNotifierAlarm,
};

use crate::{
    error::{Error, Result},
    status_to_result,
};

mod alarm;
mod periodic;
mod reactor;

pub use alarm::Alarm;
pub use periodic::Periodic;

pub fn get_time() -> Result<Duration> {
    // Possibly use a custom instant implementation?
    Ok(Duration::from_micros(unsafe {
        status_to_result!(HAL_GetFPGATime())
    }?))
}

pub fn delay(duration: Duration) -> Alarm {
    Alarm::new(duration)
}

pub(crate) struct RawNotifier {
    handle: i32,
}

impl RawNotifier {
    pub fn new(time_to_sleep: Duration) -> Result<Self> {
        let handle = unsafe { status_to_result!(HAL_InitializeNotifier()) }?;

        let raw_notifier = Self { handle };

        raw_notifier.set_time(time_to_sleep)?;

        Ok(raw_notifier)
    }

    pub fn set_time(&self, new_time: Duration) -> Result<()> {
        unsafe {
            status_to_result!(HAL_UpdateNotifierAlarm(
                self.handle,
                (get_time()? + new_time).as_micros() as u64
            ))
        }?;

        Ok(())
    }

    pub fn block_until_alarm(self) -> Result<Self> {
        let elapsed = unsafe { status_to_result!(HAL_WaitForNotifierAlarm(self.handle)) }?;

        if elapsed == 0 {
            Err(Error::NotifierStopped)
        } else {
            Ok(self)
        }
    }
}

impl Drop for RawNotifier {
    fn drop(&mut self) {
        unsafe { status_to_result!(HAL_CleanNotifier(self.handle)) }.unwrap();
    }
}
