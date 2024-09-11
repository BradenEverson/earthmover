//! Trait defining a peripheral input

/// A trait for inputs, defines a single method for polling the peripheral's data
pub trait Input {
    /// The error read_input may return
    type Error;
    /// Read bytes from peripheral entirely to an output buffer
    /// Note* This means that if the buffer is larger than one read, it will read again
    fn read_input(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

#[cfg(feature = "rpi")]
pub mod rpi_inputs;

#[cfg(feature = "jetson")]
pub mod jetson_inputs;
