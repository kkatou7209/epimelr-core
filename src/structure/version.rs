use std::fmt::Display;

use crate::specification::structure::version::validate_version;

/// PDF version representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// PDF version major number.
    major: u8,
    /// PDF version minor number.
    minor: u8,
}

impl Version {
    /// Creates a new `Version` with the given major and minor numbers.
    pub fn new(major: u8, minor: u8) -> Self {

        Self {
            major,
            minor,
        }
    }

    /// Returns the major version number.
    pub fn major(&self) -> u8 {
        self.major
    }

    /// Returns the minor version number.
    pub fn minor(&self) -> u8 {
        self.minor
    }
}

impl Default for Version {
    
    fn default() -> Self {
        Self::new(1, 7)
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
}