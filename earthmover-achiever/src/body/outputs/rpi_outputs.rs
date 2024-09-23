//! Output Implementations for Raspberry Pi Output Peripherals

use rppal::{
    gpio::{Level, OutputPin},
    i2c::I2c,
    pwm::Pwm,
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

impl Output for Pwm {
    type Error = PeripheralError;
    fn write(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        match bytes.len() {
            0..8 => self.disable()?,
            8..16 => self.set_duty_cycle(f64::from_ne_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]))?,
            _ => self.set_frequency(
                f64::from_ne_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]),
                f64::from_ne_bytes([
                    bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                    bytes[15],
                ]),
            )?,
        }

        Ok(())
    }
}
