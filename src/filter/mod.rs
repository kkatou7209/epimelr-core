mod ascii_hex_decode;

pub use ascii_hex_decode::AsciiHexDecode;

/// Trait for PDF Filters.
pub trait Filter: Sized + Send + Sync {
    /// Decodes the given data using the filter.
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, String>;
    /// Encodes the given data using the filter.
    fn encode(&self, data: &[u8]) -> Result<Vec<u8>, String>;
}