//! Output Implementations for Raspberry Pi Output Peripherals

use rppal::{
    gpio::{Level, OutputPin},
    i2c::I2c,
    spi::Spi,
    uart::Uart,
};

use crate::body::PeripheralError;

use super::Output;

impl Output for OutputPin {
    type Error = PeripheralError;
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        for byte in bytes {
            let state = match byte {
                0..=127 => Level::Low,
                _ => Level::High,
            };
            self.write(state)
        }
        Ok(())
    }
}

impl Output for I2c {
    type Error = PeripheralError;
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write(bytes)?;
        Ok(())
    }
}

impl Output for Spi {
    type Error = PeripheralError;
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write(bytes)?;
        Ok(())
    }
}

impl Output for Uart {
    type Error = PeripheralError;
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        self.write(bytes)?;
        Ok(())
    }
}
