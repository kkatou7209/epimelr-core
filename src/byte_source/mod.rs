use std::fmt::Debug;

pub mod memory;
pub mod file;

/// A trait representing a source of PDF bytes.
pub trait ByteSource: Send + Sync + Debug + 'static {

    /// Returns a slice of bytes from the source for the given range.
    fn slice(&self, range: std::ops::Range<usize>) -> &[u8];

    /// Returns the total length of the byte source.
    fn len(&self) -> usize;
}