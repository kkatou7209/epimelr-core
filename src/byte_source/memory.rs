use crate::byte_source::ByteSource;

/// A `ByteSource` implementation that holds PDF data in memory.
#[derive(Debug, Clone)]
pub struct MemoryByteSource {
    data: Vec<u8>,
}

impl MemoryByteSource {

    /// Creates a new `MemoryByteSource` from the given byte vector.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl ByteSource for MemoryByteSource {

    fn slice(&self, range: std::ops::Range<usize>) -> &[u8] {
        &self.data[range]
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}