use std::{error::Error, thread, time::Duration};

use embedded_hal::spi::{Operation, SpiDevice};
use hal::spi::{RioSPI, SPIError};

pub trait IO {
    type Error: Error + 'static;

    fn init(&mut self) -> Result<(), Self::Error>;
    fn write(&mut self, address: u8, value: u8) -> Result<(), Self::Error>;
    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error>;
    fn shutdown(&mut self) -> Result<(), Self::Error>;
}

impl IO for RioSPI {
    type Error = SPIError;

    fn init(&mut self) -> Result<(), Self::Error> {
        self.set_frequency(500000);
        self.set_mode(embedded_hal::spi::MODE_3);
        self.set_cs_active_high(false).map_err(|_| SPIError)?;

        Ok(())
    }

    fn write(&mut self, address: u8, value: u8) -> Result<(), Self::Error> {
        let command = [address | 0x80, value, get_crc(&[address | 0x80, value])];
        SpiDevice::write(self, &command)?;

        Ok(())
    }

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        let command = [
            address,
            buffer.len() as u8,
            get_crc(&[address, buffer.len() as u8]),
        ];

        SpiDevice::write(self, &command)?;

        thread::sleep(Duration::from_micros(200));

        buffer.fill(0x95);

        let mut check = [0x3E];

        let mut operations = [
            Operation::TransferInPlace(buffer),
            Operation::TransferInPlace(&mut check),
        ];

        SpiDevice::transaction(self, &mut operations)?;

        let crc = get_crc(buffer);

        if crc != check[0] || buffer[0..4].iter().all(|byte| *byte == 0) {
            return Err(SPIError);
        }

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub(crate) fn get_crc(buffer: &[u8]) -> u8 {
    let mut crc = 0;
    for val in buffer {
        crc ^= val;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc ^= 0x0091;
            }
            crc >>= 1;
        }
    }

    crc
}
