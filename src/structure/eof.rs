use std::fmt::Display;


/// A PDF End-Of-File (EOF) marker representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EOF;

impl EOF {

    /// Creates a new `EOF` marker.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EOF {
    
    fn default() -> Self {
        Self::new()
    }
}

impl Display for EOF {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%%EOF")
    }
}