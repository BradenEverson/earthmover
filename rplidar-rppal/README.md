# RpLidar A1 Rust Implementation

This project contains a Rust-based implementation for interfacing with the **RpLidar A1** device using a Raspbery Pi.

## `RpLidarA1`: RpLidar Struct Implementation
- This file defines a `RpLidarA1` struct that handles motor control and UART communication with the RpLidar A1 device.
- It contains methods for sending commands, starting scans, and continuously reading data from the LIDAR.
- The LIDAR commands are encapsulated in a `Command` enum, which includes common operations such as `Stop`, `Scan`, `GetInfo`, and more.
- **Main features include:**
  - **`send_command`**: Sends a command to the LIDAR.
  - **`run_with_callback`**: Starts the LIDAR scan and pipes data to a callback function for further processing.

## `hello_lidar`: Example Implementation for Using RpLidar A1 on a Raspberry Pi
- This file is an example implementation that demonstrates how to initialize the LIDAR on a Raspberry Pi.
- The code uses the `rppal` crate to interface with the GPIO and UART pins, setting up UART communication at a baud rate of 115,200.
- The `run_with_callback` method from `RpLidarA1` is used to handle incoming LIDAR data and parse it using a custom callback, `parse_scan_data`.
- **`parse_scan_data` function extracts the following information from LIDAR data:**
  - **Quality** of the measurement.
  - **Angle** in degrees.
  - **Distance** in millimeters.
- Only valid measurements are printed to the console for readability.

### Setup Requirements
- **Hardware**: Raspberry Pi with connected RpLidar A1 device.
- **Software**: `rppal` crate for GPIO and UART communication.
- **Pins**: The LIDAR motor control should be connected to GPIO pin 18, and UART Rx/Tx pins should be connected to the LIDAR.

### How to Use
1. Compile the project using `cargo build`.
2. Run the example using `cargo run --example hello_lidar`.
3. Ensure that the LIDAR motor pin is connected, and the UART Rx/Tx pins are correctly set up.
