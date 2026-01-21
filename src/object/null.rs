use std::sync::LazyLock;

/// A PDF Null object representation.
/// 
/// This struct implements `PartialEq`, but all `Null`
/// instances are considered unequal to each other.
#[derive(Debug, Clone, Eq)]
pub struct Null;

impl Null {
    
    /// Creates a new `Null` object.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns the byte representation of the Null object.
    pub fn as_bytes(&self) -> &[u8] {
        b"null"
    }
}

impl PartialEq for Null {

    /// Null objects are always considered unequal.
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}