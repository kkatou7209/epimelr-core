use crate::byte_source::ByteSource;

/// A byte source backed by a memory-mapped file.
#[derive(Debug)]
pub struct FileByteSource {
    mmap: memmap2::Mmap,
}

impl FileByteSource {
    
    /// Creates a new `FileByteSource` from the given file.
    pub fn new(file: &std::fs::File) -> std::io::Result<Self> {

        let mmap = unsafe { memmap2::MmapOptions::new().map(file)? };
        
        Ok(Self { mmap })
    }
}

impl ByteSource for FileByteSource {

    fn slice(&self, range: std::ops::Range<usize>) -> &[u8] {
        
        &self.mmap[range]
    }

    fn len(&self) -> usize {
        
        self.mmap.len()
    }
}