use std::fmt::Display;

use crate::specification::structure::version::validate_version;

/// PDF version representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// PDF version major number.
    major: u8,
    /// PDF version minor number.
    minor: u8,
    /// Byte representation of the version.
    bytes: Vec<u8>,
}

impl Version {
    /// Creates a new `Version` with the given major and minor numbers.
    pub fn new(major: u8, minor: u8) -> Result<Self, String> {

        if let Err(e) = validate_version(major, minor) {
            return Err(format!("Invalid version {}.{}: {}", major, minor, e));
        }

        Ok(Self {
            major,
            minor,
            bytes: format!("{}.{}", major, minor).into_bytes()
        })
    }

    /// Returns the major version number.
    pub fn major(&self) -> u8 {
        self.major
    }

    /// Returns the minor version number.
    pub fn minor(&self) -> u8 {
        self.minor
    }

    /// Returns the byte representation of the version.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Default for Version {
    
    fn default() -> Self {
        Self::new(1, 7).unwrap()
    }
}

impl Display for Version {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%PDF-{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn test_version_creation() {
        let version = Version::new(1, 7).unwrap();
        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 7);
        assert_eq!(version.as_bytes(), b"1.7");
    }
}