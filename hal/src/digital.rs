use embedded_hal::digital::v2::{InputPin, OutputPin};
use robotrs::error::HalError;
use std::{marker::PhantomData, ptr};

pub struct RioPin<T> {
    data: PhantomData<T>,
    handle: hal_sys::HAL_PortHandle,
}

pub struct Input;
pub struct Output;

impl<T> RioPin<T> {
    pub fn new_input(channel: u8) -> Result<RioPin<Input>, HalError> {
        let mut error = 0;

        let handle;
        unsafe {
            handle = hal_sys::HAL_GetPort(channel as i32);
            hal_sys::HAL_InitializeDIOPort(handle, 1, ptr::null(), &mut error);
        }

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
    type Error = HalError;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let mut error = 0;
        unsafe { hal_sys::HAL_SetDIO(self.handle, 0, &mut error) };

        if error != 0 {
            Err(HalError::from_raw(error))
        } else {
            Ok(())
        }
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let mut error = 0;
        unsafe { hal_sys::HAL_SetDIO(self.handle, 1, &mut error) };

        if error != 0 {
            Err(HalError::from_raw(error))
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
    type Error = HalError;

    fn is_high(&self) -> Result<bool, Self::Error> {
        let mut error = 0;
        let is_high = unsafe { hal_sys::HAL_GetDIO(self.handle, &mut error) };

        if error != 0 {
            Err(HalError::from_raw(error))
        } else {
            Ok(if is_high == 1 { true } else { false })
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(!self.is_high()?)
    }
}
