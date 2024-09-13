//! Extensions of the Peripheral Enum to construct Raspberry Pi Peripherals

use rppal::{
    gpio::Gpio,
    i2c::I2c,
    pwm::{Channel, Pwm},
    spi::{Bus, Mode, SlaveSelect, Spi},
};

use super::Peripheral;

impl Peripheral {
    /// Creates a Peripheral Input Node for a Raspberry Pi GPIO pin
    pub fn gpio_input(pin: u8) -> rppal::gpio::Result<Self> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(pin)?.into_input();

        Ok(Self::Input(Box::new(pin)))
    }

    /// Creates a Peripheral Output Node for a Raspberry Pi GPIO pin
    pub fn gpio_output(pin: u8) -> rppal::gpio::Result<Self> {
        let gpio = Gpio::new()?;
        let pin = gpio.get(pin)?.into_output();

        Ok(Self::Output(Box::new(pin)))
    }

    /// Creates a Peripheral Iput Node for a Raspberry Pi i2c channel
    pub fn i2c_input(address: u16) -> rppal::i2c::Result<Self> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(address)?;

        Ok(Self::Input(Box::new(i2c)))
    }

    /// Creates a Peripheral Output Node for a Raspberry Pi i2c channel
    pub fn i2c_output(address: u16) -> rppal::i2c::Result<Self> {
        let mut i2c = I2c::new()?;
        i2c.set_slave_address(address)?;

        Ok(Self::Input(Box::new(i2c)))
    }

    /// Creates a Peripheral Input Node for a Raspberry Pi SPI Bus
    pub fn spi_input(
        bus: Bus,
        slave_select: SlaveSelect,
        clock_speed: u32,
        mode: Mode,
    ) -> rppal::spi::Result<Self> {
        let spi = Spi::new(bus, slave_select, clock_speed, mode)?;

        Ok(Self::Input(Box::new(spi)))
    }

    /// Creates a Peripheral Output Node for a Raspberry Pi SPI Bus
    pub fn spi_output(
        bus: Bus,
        slave_select: SlaveSelect,
        clock_speed: u32,
        mode: Mode,
    ) -> rppal::spi::Result<Self> {
        let spi = Spi::new(bus, slave_select, clock_speed, mode)?;

        Ok(Self::Output(Box::new(spi)))
    }

    /// Creates a Peripheral Output Node for a Raspberry Pi PWM Channel
    pub fn pwm_output(channel: Channel) -> rppal::pwm::Result<Self> {
        let pwm = Pwm::new(channel)?;
        Ok(Self::Output(Box::new(pwm)))
    }
}
