use std::fmt::Display;
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
}

impl PartialEq for Null {

    /// Null objects are always considered unequal.
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Display for Null {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}