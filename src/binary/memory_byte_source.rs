use std::ops::{Index, Range, RangeFull};
use std::process::Output;

/// A byte source backed by an in-memory vector of bytes.
#[derive(Debug, Clone)]
pub struct MemoryByteSource {
    data: Vec<u8>,
}

impl MemoryByteSource {
    /// Creates a new `MemoryByteSource` from the given vector of bytes.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl Index<usize> for MemoryByteSource {

    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.data[index]
    }
}

impl Index<Range<usize>> for MemoryByteSource {

    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl Index<RangeFull> for MemoryByteSource {

    type Output = [u8];

    fn index(&self, _index: RangeFull) -> &Self::Output {
        &self.data[..]
    }
}

impl super::ByteSource for MemoryByteSource {
    fn len(&self) -> usize {
        self.data.len()
    }
}