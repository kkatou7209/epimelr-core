use crate::specification::object::real::is_valid_real_number_bytes;

/// A PDF Real object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Real {
    bytes: Vec<u8>,
}

impl Real {
    
    /// Creates a new `Real` from the given bytes.
    pub fn new(bytes: &[u8]) -> Self {
        
        if !is_valid_real_number_bytes(bytes) {
            panic!("real number contains invalid characters");
        }

        Self { bytes: bytes.to_vec() }
    }

    /// Returns the byte representation of the Real.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}