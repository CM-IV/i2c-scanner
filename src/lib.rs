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

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    pub fn contains_address(&self, address: u8) -> bool {
        self.devices.iter().any(|device| device.address == address)
    }

    pub fn find_in_range(&self, start: u8, end: u8) -> impl Iterator<Item = &I2CDevice> {
        self.devices
            .iter()
            .filter(move |device| device.address >= start && device.address <= end)
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

        for address in 1..128u8 {
            if self.check_address_internal(address)? {
                result.devices.push(I2CDevice { address }).unwrap_or(());
            }
        }

        Ok(result)
    }

    pub fn scan_range(&mut self, start: u8, end: u8) -> Result<ScanResult, I2C::Error> {
        let start = start.clamp(1, 127);
        let end = end.clamp(1, 127);

        let mut result = ScanResult {
            devices: heapless::Vec::new(),
        };

        for address in start..=end {
            if self.check_address_internal(address)? {
                result.devices.push(I2CDevice { address }).unwrap_or(());
            }
        }

        Ok(result)
    }

    fn check_address_internal(&mut self, address: u8) -> Result<bool, I2C::Error> {
        match self.i2c.write(address, &[]) {
            Ok(_) => Ok(true),
            Err(_) => {
                // This is a bit of a hack, but most I2C implementations will return
                // a specific error for "no ACK received" or "NACK" which means no device at this address.
                // We want to treat this as "no device" rather than a real error.
                Ok(false)
            }
        }
    }

    pub fn check_address(&mut self, address: u8) -> Result<bool, I2C::Error> {
        if address > 127 {
            // Invalid 7-bit address
            return Ok(false);
        }

        self.check_address_internal(address)
    }

    pub fn release(self) -> I2C {
        self.i2c
    }
}
