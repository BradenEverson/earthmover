//! Special structs for marking a buffer with a special position, allows us to fill a buffer and
//! keep track of our spot without using a `Vec<u8>`

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// A Buffer position marker
pub struct BufferMarker<const BUFFER_SIZE: usize>(usize);

impl<const BUFFER_SIZE: usize> BufferMarker<BUFFER_SIZE> {
    /// Increment the buffer marker, returning None if the buffer is full
    pub fn inc(&mut self) -> Option<usize> {
        if self.0 + 1 >= BUFFER_SIZE {
            self.0 = 0;
            None
        } else {
            self.0 += 1;
            Some(self.0)
        }
    }
}
