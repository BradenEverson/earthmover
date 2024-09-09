//! Trait defining a peripheral input

/// A trait for inputs, defines a single method for polling the peripheral's data
pub trait Input {
    /// Read bytes from peripheral to output
    fn read(&mut self) -> Vec<u8>;
}
