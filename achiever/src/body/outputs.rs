//! Trait defining a peripheral output

/// A trait for outputs, defines a single method for writing bytes to an output
pub trait Output {
    /// Write bytes to an output peripheral
    fn write(&mut self, bytes: &[u8]);
}

#[cfg(feature = "rpi")]
pub mod rpi_outputs;

#[cfg(feature = "jetson")]
pub mod jetson_outputs;
