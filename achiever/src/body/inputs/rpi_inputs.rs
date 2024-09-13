//! Input Implementations for Raspberry Pi Peripherals

use rppal::{gpio::InputPin, i2c::I2c, spi::Spi, uart::Uart};

use crate::body::PeripheralError;

use super::Input;

impl Input for InputPin {
    type Error = PeripheralError;
    fn read_input(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        for loc in buffer {
            *loc = self.read() as u8
        }
        Ok(())
    }
}

impl Input for I2c {
    type Error = PeripheralError;
    fn read_input(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.read(buf)?;
        Ok(())
    }
}

impl Input for Spi {
    type Error = PeripheralError;
    fn read_input(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.read(buf)?;
        Ok(())
    }
}

impl Input for Uart {
    type Error = rppal::uart::Error;
    fn read_input(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.read(buf)?;
        Ok(())
    }
}
