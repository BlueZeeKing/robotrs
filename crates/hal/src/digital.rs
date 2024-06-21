use embedded_hal::digital::{Error, ErrorKind, ErrorType, InputPin, OutputPin};
use robotrs::error::HalError;
use std::{marker::PhantomData, ptr};
use tracing::debug;

pub struct RioPin<T> {
    data: PhantomData<T>,
    handle: hal_sys::HAL_PortHandle,
}

pub struct Input;
pub struct Output;

#[derive(Debug)]
pub struct DigitalError(pub HalError);

impl std::fmt::Display for DigitalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for DigitalError {}

impl From<HalError> for DigitalError {
    fn from(value: HalError) -> Self {
        Self(value)
    }
}

impl Error for DigitalError {
    fn kind(&self) -> embedded_hal::digital::ErrorKind {
        ErrorKind::Other
    }
}

impl<T> ErrorType for RioPin<T> {
    type Error = DigitalError;
}

impl<T> RioPin<T> {
    pub fn new_input(channel: u8) -> Result<RioPin<Input>, HalError> {
        let mut error = 0;

        let handle = unsafe {
            hal_sys::HAL_InitializeDIOPort(
                hal_sys::HAL_GetPort(channel as i32),
                1,
                ptr::null(),
                &mut error,
            )
        };

        if error != 0 {
            Err(HalError::from_raw(error))
        } else {
            Ok(RioPin::<Input> {
                data: PhantomData,
                handle,
            })
        }
    }

    pub fn new_output(channel: u8) -> Result<RioPin<Output>, HalError> {
        let mut error = 0;

        let handle;
        unsafe {
            handle = hal_sys::HAL_GetPort(channel as i32);
            hal_sys::HAL_InitializeDIOPort(handle, 0, ptr::null(), &mut error);
        }

        if error != 0 {
            Err(HalError::from_raw(error))
        } else {
            Ok(RioPin::<Output> {
                data: PhantomData,
                handle,
            })
        }
    }
}

impl OutputPin for RioPin<Output> {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        let mut error = 0;
        unsafe { hal_sys::HAL_SetDIO(self.handle, 0, &mut error) };

        if error != 0 {
            Err(HalError::from_raw(error).into())
        } else {
            Ok(())
        }
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let mut error = 0;
        unsafe { hal_sys::HAL_SetDIO(self.handle, 1, &mut error) };

        if error != 0 {
            Err(HalError::from_raw(error).into())
        } else {
            Ok(())
        }
    }
}

impl<T> Drop for RioPin<T> {
    fn drop(&mut self) {
        unsafe { hal_sys::HAL_FreeDIOPort(self.handle) }
    }
}

impl InputPin for RioPin<Input> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        let mut error = 0;
        let is_high = unsafe { hal_sys::HAL_GetDIO(self.handle, &mut error) };

        if error != 0 {
            Err(HalError::from_raw(error).into())
        } else {
            Ok(is_high == 1)
        }
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}

impl RioPin<Input> {
    pub async fn wait_for_high(&mut self) -> Result<(), DigitalError> {
        loop {
            println!("Starting");
            let is_high = dbg!(self.is_high())?;

            debug!(is_high);

            if is_high {
                return Ok(());
            }

            robotrs::yield_now().await;
        }
    }

    pub async fn wait_for_low(&mut self) -> Result<(), DigitalError> {
        loop {
            if self.is_low()? {
                return Ok(());
            }

            robotrs::yield_now().await;
        }
    }
}
