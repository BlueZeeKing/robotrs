use embedded_hal::spi::{Error, ErrorKind, ErrorType, SpiBus};
use hal_sys::{
    HAL_CloseSPI, HAL_InitializeSPI, HAL_ReadSPI, HAL_SPIPort, HAL_SPIPort_HAL_SPI_kMXP,
    HAL_TransactionSPI, HAL_WriteSPI,
};
use robotrs::error::HalError;

pub struct RioSPI {
    port: i32,
}

impl RioSPI {
    pub fn new_mxp() -> Result<Self, HalError> {
        Self::new(HAL_SPIPort_HAL_SPI_kMXP)
    }

    fn new(port: HAL_SPIPort) -> Result<Self, HalError> {
        let mut status = 0;

        unsafe {
            HAL_InitializeSPI(port, &mut status);
        }

        if status != 0 {
            return Err(HalError::from_raw(status));
        }

        Ok(Self { port })
    }
}

#[derive(Debug)]
pub struct SPIError;

impl Error for SPIError {
    fn kind(&self) -> embedded_hal::spi::ErrorKind {
        ErrorKind::Other // TODO: Make this work better
    }
}

impl ErrorType for RioSPI {
    type Error = SPIError;
}

impl SpiBus for RioSPI {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let count = unsafe { HAL_ReadSPI(self.port, words.as_mut_ptr(), words.len() as i32) };

        if count == -1 {
            Err(SPIError)
        } else {
            Ok(())
        }
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let count = unsafe { HAL_WriteSPI(self.port, words.as_ptr(), words.len() as i32) };

        if count == -1 {
            Err(SPIError)
        } else {
            Ok(())
        }
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        if read.len() != write.len() {
            return Err(SPIError);
        }

        let count = unsafe {
            HAL_TransactionSPI(
                self.port,
                write.as_ptr(),
                read.as_mut_ptr(),
                read.len() as i32,
            )
        };

        if count == -1 {
            Err(SPIError)
        } else {
            Ok(())
        }
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let data_copy = words.to_vec(); // Might not be needed

        let count = unsafe {
            HAL_TransactionSPI(
                self.port,
                data_copy.as_ptr(),
                words.as_mut_ptr(),
                words.len() as i32,
            )
        };

        if count == -1 {
            Err(SPIError)
        } else {
            Ok(())
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Drop for RioSPI {
    fn drop(&mut self) {
        unsafe { HAL_CloseSPI(self.port) }
    }
}
