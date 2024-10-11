//! RpLidar Struct for Handling Motor Control and UART Communication

use std::time::Duration;

use rppal::{gpio::OutputPin, uart::Uart};

/// An RpLidar A1 Device
pub struct RpLidarA1 {
    /// The UART channel connected to the RpLidar
    uart: Uart,
    /// A GPIO output pin connected to the RpLidar's motor
    motor: OutputPin,
}

/// RpLidar A1 Command Types
#[repr(u8)]
pub enum Command {
    /// Stop the current scan
    Stop = 0x25,
    /// Reset the current scan
    Reset = 0x40,
    /// Begin scanning
    Scan = 0x20,
    /// Enter the scanning state and working at the highest speed
    ExpressScan = 0x82,
    /// Enter the scanning state and force data output without checking rotation speed
    ForceScan = 0x21,
    /// Send out the device info (e.g. serial number)
    GetInfo = 0x50,
    /// SEnd out the device health info
    GetHealth = 0x52,
    /// Send out single sampling rate
    GetSampleRate = 0x59,
    /// Get LIDAR configuration
    GetLidarConf = 0x84,
}

impl RpLidarA1 {
    /// Creates a new RpLidar from already constructed UART and OutputPin
    pub fn new(uart: Uart, motor: OutputPin) -> Self {
        Self { uart, motor }
    }

    /// Send a command to the RpLidar
    pub fn send_command(&mut self, command: Command) {
        let cmd_packet = [0xA5, command as u8];
        self.uart
            .write(&cmd_packet)
            .expect("Failed to send command");
    }

    /// Continously reads lidar data until a failure happens. Pipes the data through to a
    /// `response_callback` function that does something to a `&[u8]` allowing fine-grained control
    /// over data response
    pub fn run_with_callback<FN: Fn(&[u8])>(mut self, response_callback: FN) {
        self.motor.set_high();

        self.send_command(Command::Reset);
        std::thread::sleep(Duration::from_millis(500));

        self.send_command(Command::Scan);

        let mut buffer: [u8; 255] = [0; 255];
        loop {
            match self.uart.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => response_callback(&buffer),
                _ => break,
            }
        }

        self.motor.set_low();
        self.send_command(Command::Stop);
    }
}
