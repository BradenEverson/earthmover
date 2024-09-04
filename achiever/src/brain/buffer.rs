#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BufferMarker<const BUFFER_SIZE: usize>(usize);

impl<const BUFFER_SIZE: usize> BufferMarker<BUFFER_SIZE> {
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

