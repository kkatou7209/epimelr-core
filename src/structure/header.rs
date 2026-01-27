use std::fmt::Display;

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
    pub fn new(version: Version, byte_marker: ByteMarker) -> Self {
        Self {
            version,
            byte_marker,
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

impl Display for Header {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.version, self.byte_marker)
    }
}

impl Default for Header {
    
    fn default() -> Self {
        Self {
            version: Version::default(),
            byte_marker: ByteMarker::default(),
        }
    }
}