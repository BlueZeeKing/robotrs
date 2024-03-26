use std::error::Error;

use embedded_hal::spi::{Operation, SpiDevice};
use hal::spi::{RioSPI, SPIError};

pub trait IO {
    type Error: Error;

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

        let mut check = [0x3E];

        let mut operations = [
            Operation::Write(&command),
            Operation::DelayNs(1000000),
            Operation::Read(buffer),
            Operation::Read(&mut check),
        ];

        SpiDevice::transaction(self, &mut operations)?;

        let crc = get_crc(buffer);

        if crc != check[0] || buffer[0..4].iter().all(|byte| *byte == 0) {
            return Err(SPIError);
        }

        SpiDevice::read(self, buffer)?;

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn get_crc(data: &[u8]) -> u8 {
    let mut crc = 0;

    for byte in data {
        crc ^= 0x00ff & byte;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc ^= 145;
            }
            crc = crc.wrapping_shr(1);
        }
    }

    crc
}
