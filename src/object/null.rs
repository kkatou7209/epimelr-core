/// A PDF Null object representation.
#[derive(Debug, Clone)]
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