use defer_lite::defer;
use embedded_hal::digital::{Error, ErrorKind, ErrorType, InputPin, OutputPin};
use hal_sys::{
    HAL_AnalogTriggerType_HAL_Trigger_kState, HAL_CleanInterrupts, HAL_InitializeInterrupts,
    HAL_InterruptHandle, HAL_ReleaseWaitingInterrupt, HAL_RequestInterrupts,
    HAL_SetInterruptUpSourceEdge, HAL_WaitForInterrupt,
};
use robotrs::error::HalError;
use std::{marker::PhantomData, ptr, thread};
use tracing::{trace, warn};

pub struct RioPin<T> {
    data: PhantomData<T>,
    handle: hal_sys::HAL_PortHandle,
    interrupt: Option<HAL_InterruptHandle>,
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
                interrupt: None,
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
                interrupt: None,
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
        if let Some(interrupt) = self.interrupt {
            unsafe { HAL_CleanInterrupts(interrupt) };
        }

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
    pub async fn wait_for_edge(&mut self, on_high: bool, on_low: bool) -> Result<(), DigitalError> {
        let mut status = 0;

        let interrupt = if let Some(interrupt) = self.interrupt {
            interrupt
        } else {
            let interrupt = unsafe { HAL_InitializeInterrupts(&mut status) };
            if status != 0 {
                return Err(HalError::from_raw(status).into());
            }

            unsafe {
                HAL_RequestInterrupts(
                    interrupt,
                    self.handle,
                    HAL_AnalogTriggerType_HAL_Trigger_kState,
                    &mut status,
                )
            };
            if status != 0 {
                return Err(HalError::from_raw(status).into());
            }

            self.interrupt = Some(interrupt);

            interrupt
        };

        unsafe {
            HAL_SetInterruptUpSourceEdge(
                interrupt,
                if on_high { 1 } else { 0 },
                if on_low { 1 } else { 0 },
                &mut status,
            )
        };
        if status != 0 {
            return Err(HalError::from_raw(status).into());
        }

        let (sender, receiver) = oneshot::channel();

        let join_handle = thread::Builder::new()
            .name("Waiting".to_string())
            .spawn(move || {
                let mut status = 0;
                unsafe { HAL_WaitForInterrupt(interrupt, f64::MAX, 1, &mut status) };
                let _ = sender.send(if status != 0 {
                    Err(HalError::from_raw(status))
                } else {
                    Ok(())
                });
            })
            .unwrap();

        defer! {
            let mut status = 0;
            unsafe { HAL_ReleaseWaitingInterrupt(interrupt, &mut status) };
            if status != 0 {
                warn!("Failed to release the waiting interrupt");
            }

            trace!("Closing thread");

            join_handle.join().expect("DIO waiting thread crashed");

            trace!("Thread has closed");
        }

        receiver.await.expect("DIO waiting thread crashed")?;

        Ok(())
    }

    pub async fn wait_for_high(&mut self) -> Result<(), DigitalError> {
        self.wait_for_edge(true, false).await
    }

    pub async fn wait_for_low(&mut self) -> Result<(), DigitalError> {
        self.wait_for_edge(false, true).await
    }
}
