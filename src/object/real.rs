use crate::specification::object::real::validate_real_number_bytes;

/// A PDF Real object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Real {
    bytes: Vec<u8>,
}

impl Real {
    
    /// Creates a new `Real` from the given bytes.
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        
        if let Err(e) = validate_real_number_bytes(bytes) {
            return Err(format!("Real number contains invalid characters: {}", e));
        }

        Ok(Self { bytes: bytes.to_vec() })
    }

    /// Returns the byte representation of the Real.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}