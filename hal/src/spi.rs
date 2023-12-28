use std::u32;

use embedded_hal::{
    delay::DelayNs,
    spi::{Error, ErrorKind, ErrorType, SpiDevice, MODE_0, MODE_1, MODE_2, MODE_3},
};
use hal_sys::{
    HAL_CloseSPI, HAL_InitializeSPI, HAL_ReadSPI, HAL_SPIPort, HAL_SetSPIChipSelectActiveHigh,
    HAL_SetSPIChipSelectActiveLow, HAL_SetSPIMode, HAL_SetSPISpeed, HAL_TransactionSPI,
    HAL_WriteSPI,
};
use robotrs::error::HalError;

use crate::time::Delay;

pub struct RioSPI {
    port: i32,
    timer: Option<Delay>,
}

/// Maximum onboard port number is 3, anything higher will panic
#[derive(Clone, Copy)]
pub enum Port {
    MXP,
    Onboard(u8),
}

impl Into<HAL_SPIPort> for Port {
    fn into(self) -> HAL_SPIPort {
        match self {
            Port::MXP => hal_sys::HAL_SPIPort_HAL_SPI_kMXP,
            Port::Onboard(n) => match n {
                0 => hal_sys::HAL_SPIPort_HAL_SPI_kOnboardCS0,
                1 => hal_sys::HAL_SPIPort_HAL_SPI_kOnboardCS1,
                2 => hal_sys::HAL_SPIPort_HAL_SPI_kOnboardCS2,
                3 => hal_sys::HAL_SPIPort_HAL_SPI_kOnboardCS3,
                _ => panic!("Unknown spi port number"),
            },
        }
    }
}

impl RioSPI {
    pub fn new(port: Port) -> Result<Self, HalError> {
        let mut status = 0;

        unsafe {
            HAL_InitializeSPI(port.into(), &mut status);
        }

        if status != 0 {
            return Err(HalError::from_raw(status));
        }

        Ok(Self {
            port: port.into(),
            timer: None,
        })
    }

    pub fn set_mode(&mut self, mode: embedded_hal::spi::Mode) {
        let mode = if mode == MODE_0 {
            hal_sys::HAL_SPIMode_HAL_SPI_kMode0
        } else if mode == MODE_1 {
            hal_sys::HAL_SPIMode_HAL_SPI_kMode1
        } else if mode == MODE_2 {
            hal_sys::HAL_SPIMode_HAL_SPI_kMode2
        } else if mode == MODE_3 {
            hal_sys::HAL_SPIMode_HAL_SPI_kMode3
        } else {
            unreachable!()
        };

        unsafe {
            HAL_SetSPIMode(self.port, mode);
        }
    }

    pub fn set_frequency(&mut self, freq: u32) {
        unsafe {
            HAL_SetSPISpeed(self.port, freq as i32);
        }
    }

    pub fn set_cs_active(&mut self, is_high: bool) -> Result<(), HalError> {
        let mut status = 0;
        if is_high {
            unsafe {
                HAL_SetSPIChipSelectActiveHigh(self.port, &mut status);
            }
        } else {
            unsafe {
                HAL_SetSPIChipSelectActiveLow(self.port, &mut status);
            }
        }

        if status == 0 {
            Ok(())
        } else {
            Err(HalError::from_raw(status))
        }
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

impl SpiDevice for RioSPI {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        let count = unsafe { HAL_ReadSPI(self.port, words.as_mut_ptr(), words.len() as i32) };

        if count == -1 || count != words.len() as i32 {
            Err(SPIError)
        } else {
            Ok(())
        }
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        let count = unsafe { HAL_WriteSPI(self.port, words.as_ptr(), words.len() as i32) };

        if count == -1 || count != words.len() as i32 {
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

        if count == -1 || count != read.len() as i32 {
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

        if count == -1 || count != words.len() as i32 {
            Err(SPIError)
        } else {
            Ok(())
        }
    }

    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        for operation in operations {
            match operation {
                embedded_hal::spi::Operation::Read(words) => self.read(words)?,
                embedded_hal::spi::Operation::Write(words) => self.write(words)?,
                embedded_hal::spi::Operation::Transfer(read, write) => {
                    self.transfer(read, write)?
                }
                embedded_hal::spi::Operation::TransferInPlace(words) => {
                    self.transfer_in_place(words)?
                }
                embedded_hal::spi::Operation::DelayNs(nanos) => {
                    if self.timer.is_none() {
                        self.timer = Some(Delay::new().map_err(|_| SPIError)?);
                    }

                    self.timer.as_mut().unwrap().delay_ns(*nanos);
                }
            }
        }

        Ok(())
    }
}

impl Drop for RioSPI {
    fn drop(&mut self) {
        unsafe { HAL_CloseSPI(self.port) }
    }
}
