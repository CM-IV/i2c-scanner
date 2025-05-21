#![no_std]

use embedded_hal::i2c::{I2c, SevenBitAddress};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I2CDevice {
    pub address: u8,
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub devices: heapless::Vec<I2CDevice, 128>,
}

impl ScanResult {
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
    pub fn has_device_at(&self, address: u8) -> bool {
        self.devices.iter().any(|dev| dev.address == address)
    }
}

pub struct I2CScanner<I2C> {
    i2c: I2C,
}

impl<I2C> I2CScanner<I2C>
where
    I2C: I2c<SevenBitAddress>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn scan(&mut self) -> Result<ScanResult, I2C::Error> {
        let mut result = ScanResult {
            devices: heapless::Vec::new(),
        };

        for addr in 1..128u8 {
            match self.i2c.write(addr, &[]) {
                Ok(_) => {
                    result
                        .devices
                        .push(I2CDevice { address: addr })
                        .unwrap_or(());
                }
                Err(_) => {}
            }
        }

        Ok(result)
    }

    pub fn check_address(&mut self, address: u8) -> Result<bool, I2C::Error> {
        if address > 127 {
            return Ok(false);
        }

        match self.i2c.write(address, &[]) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn release(self) -> I2C {
        self.i2c
    }
}
