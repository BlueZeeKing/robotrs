use embedded_hal::i2c::{Error, ErrorKind, ErrorType, I2c, Operation, SevenBitAddress};
use hal_sys::{HAL_CloseI2C, HAL_I2CPort, HAL_InitializeI2C, HAL_ReadI2C, HAL_WriteI2C};
use robotrs::error::HalError;

pub struct RioI2C {
    port: HAL_I2CPort,
}

#[derive(Clone, Copy)]
pub enum Port {
    MXP,
    /// This port should not be used
    Onboard,
}

impl From<Port> for HAL_I2CPort {
    fn from(value: Port) -> Self {
        match value {
            Port::MXP => hal_sys::HAL_I2CPort_HAL_I2C_kMXP,
            Port::Onboard => hal_sys::HAL_I2CPort_HAL_I2C_kOnboard,
        }
    }
}

impl RioI2C {
    pub fn new(port: Port) -> Result<Self, HalError> {
        let mut status = 0;

        unsafe {
            HAL_InitializeI2C(port.into(), &mut status);
        }

        if status != 0 {
            return Err(HalError::from_raw(status));
        }

        Ok(Self { port: port.into() })
    }
}

#[derive(Debug)]
pub struct I2CError;

impl std::fmt::Display for I2CError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "I2C error")
    }
}

impl std::error::Error for I2CError {}

impl Error for I2CError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other // TODO: Make this work better
    }
}

impl ErrorType for RioI2C {
    type Error = I2CError;
}

impl I2c<SevenBitAddress> for RioI2C {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            let res = match operation {
                Operation::Read(buf) => unsafe {
                    HAL_ReadI2C(
                        self.port,
                        address as i32,
                        buf.as_mut_ptr(),
                        buf.len() as i32,
                    )
                },
                Operation::Write(buf) => unsafe {
                    HAL_WriteI2C(self.port, address as i32, buf.as_ptr(), buf.len() as i32)
                },
            };

            if res == -1 {
                return Err(I2CError);
            }
        }

        Ok(())
    }
}

impl Drop for RioI2C {
    fn drop(&mut self) {
        unsafe { HAL_CloseI2C(self.port) }
    }
}
