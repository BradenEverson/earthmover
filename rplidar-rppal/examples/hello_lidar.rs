//! Example for running off a raspberry pi with UART Rx and Tx pins connected to the RpLidar and
//! the motor signal at gpio pin 18

use rplidar::rplidar_a1::RpLidarA1;
use rppal::gpio::Gpio;
use rppal::uart::{Parity, Uart};
use std::time::Duration;

/// The motor pin for spinning the lidar
pub const MOTOR_CONTROL_PIN: u8 = 18;

fn main() {
    let gpio = Gpio::new().expect("Failed to setup GPIO");
    let motor_pin = gpio
        .get(MOTOR_CONTROL_PIN)
        .expect("Failed to establish motor pin")
        .into_output();

    let mut uart = Uart::new(115_200, Parity::None, 8, 1).expect("Failed to initialize UART");
    uart.set_read_mode(255, Duration::from_millis(500))
        .expect("Failed to set timeout");

    let lidar = RpLidarA1::new(uart, motor_pin);
    lidar.run_with_callback(parse_scan_data);
}

/// Scans the lidar data and prints the readings to the console
pub fn parse_scan_data(data: &[u8]) {
    if data.len() < 5 {
        return;
    }

    let mut index = 0;
    while index + 5 <= data.len() {
        let quality = (data[index] >> 2) & 0x3F; // Extract quality (upper 6 bits)
        let raw_angle = ((data[index + 2] as u16) << 8 | data[index + 1] as u16) >> 1;
        let angle_in_degrees = (raw_angle as f32) / 64.0; // Convert to degrees

        let raw_distance = (data[index + 4] as u16) << 8 | data[index + 3] as u16;
        let distance_in_mm = raw_distance as f32 / 4.0; // Convert to mm

        if quality > 10 && raw_distance > 0 && distance_in_mm < 12000.0 {
            println!(
                "Quality: {}, Angle: {:.2}°, Distance: {:.2} mm",
                quality, angle_in_degrees, distance_in_mm
            );
        } else {
            println!(
                "Invalid measurement or noisy data (Quality: {}, Distance: {})",
                quality, distance_in_mm
            );
        }

        index += 5;
    }
}
