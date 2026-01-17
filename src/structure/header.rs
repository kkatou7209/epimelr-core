use crate::structure::byte_marker::ByteMarker;
use crate::structure::version::Version;

/// PDF Header representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// PDF version of the header.
    version: Version,
    /// Byte marker of the header.
    byte_marker: ByteMarker,
}

impl Header {

    /// Creates a new `Header` with the given PDF version.
    pub fn new(version: Version) -> Self {
        Self {
            version,
            // Traditional byte marker "%âãÏÓ"
            byte_marker: ByteMarker::new(b"\xE2\xE3\xCF\xD3".to_vec()),
        }
    }

    /// Returns the PDF version of the header.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the byte marker of the header.
    pub fn byte_marker(&self) -> &ByteMarker {

        &self.byte_marker
    }
}

