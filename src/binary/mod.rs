//! Module for binary data handling.
mod memory_byte_source;

use std::{fmt::Debug, fs::File, io::{BufReader, Read}, ops::{Index, Range, RangeFull}};

/// A trait representing a source of bytes that can be indexed by `usize`.
/// 
/// The data source allows read-only access to its bytes.
pub(crate) trait ByteSource:
    Index<usize, Output = u8> +
    Index<Range<usize>, Output = [u8]> +
    Index<RangeFull, Output = [u8]> +
    Debug + Send + Sync {
        
    /// Returns the length of the byte source.
    fn len(&self) -> usize;
}

/// Creates a `ByteSource` from a file.
pub fn create_source_from_file(file: &File) -> std::result::Result<impl ByteSource +'static, Box<dyn std::error::Error>> {
    
    let mut buf = BufReader::new(file);

    let mut data = Vec::new();

    buf.read_to_end(&mut data)?;

    Ok(memory_byte_source::MemoryByteSource::new(data))
}