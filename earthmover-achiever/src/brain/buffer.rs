//! Special structs for marking a buffer with a special position, allows us to fill a buffer and
//! keep track of our spot without using a `Vec<u8>`

use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
/// A DataBuffer with known position
pub struct DataBuffer<const BUFFER_SIZE: usize> {
    /// The data
    data: [f32; BUFFER_SIZE],
    /// Where we are in that data
    marker: BufferMarker<BUFFER_SIZE>,
}

impl<const BUFFER_SIZE: usize> Default for DataBuffer<BUFFER_SIZE> {
    fn default() -> Self {
        Self {
            data: [0f32; BUFFER_SIZE],
            marker: BufferMarker::default(),
        }
    }
}

impl<const BUFFER_SIZE: usize> AsRef<[f32]> for DataBuffer<BUFFER_SIZE> {
    fn as_ref(&self) -> &[f32] {
        if *self.marker == BUFFER_SIZE - 1 {
            &self.data
        } else {
            &self.data[0..*self.marker]
        }
    }
}

impl<const BUFFER_SIZE: usize> DataBuffer<BUFFER_SIZE> {
    /// Adds a buffer of bytes to the internal data buffer. If the length of the data to add is
    /// greater than what's left in the buffer, we close early and return `None`
    pub fn add_data(&mut self, buf: &[f32]) -> Option<()> {
        if buf.len() <= self.data.len() - *self.marker {
            for byte in buf {
                *self.get()? = *byte;
            }
            Some(())
        } else {
            None
        }
    }

    pub fn export(&self) -> Vec<f32> {
        self.data[0..(*self.marker)].to_vec()
    }

    /// Returns a mutable reference to the current byte, then increments the marker
    fn get(&mut self) -> Option<&mut f32> {
        let mut_byte = &mut self.data[*self.marker];
        let _ = self.marker.inc();
        Some(mut_byte)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A Buffer position marker
pub struct BufferMarker<const BUFFER_SIZE: usize>(usize);

impl<const BUFFER_SIZE: usize> BufferMarker<BUFFER_SIZE> {
    /// Increment the buffer marker, returning None if the buffer is full
    pub fn inc(&mut self) -> Option<usize> {
        if self.0 + 1 >= BUFFER_SIZE {
            None
        } else {
            self.0 += 1;
            Some(self.0)
        }
    }
    /// Reset a buffer marker
    pub fn rst(&mut self) {
        self.0 = 0
    }
}

impl<const BUFFER_SIZE: usize> Deref for BufferMarker<BUFFER_SIZE> {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::DataBuffer;

    #[test]
    fn data_buffer_can_be_fully_filled() {
        let mut buf: DataBuffer<10> = DataBuffer::default();
        let data = [10f32; 10];

        buf.add_data(&data).expect("Fill buf all the way with data");
        assert_eq!(buf.as_ref(), &data);
    }

    #[test]
    fn data_buffer_cannot_be_overfilled() {
        let mut buf: DataBuffer<10> = DataBuffer::default();
        let data = [10f32; 11];

        let overfill = buf.add_data(&data);
        assert!(overfill.is_none())
    }

    #[test]
    fn partially_filled_data_buffer() {
        let mut buf: DataBuffer<100> = DataBuffer::default();
        let small_chunk = [1f32, 2f32];
        buf.add_data(&small_chunk).expect("Add to a buffer");
        let word: &[f32] = buf.as_ref();

        assert_eq!(word, small_chunk)
    }
}
