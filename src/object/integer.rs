use crate::specification::object::integer::validate_integer_bytes;

/// PDF Integer object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    bytes: Vec<u8>,
}

impl Integer {

    /// Creates a new `Integer` from the given bytes.
    pub fn new(bytes: &[u8]) -> Result<Self, String> {

        if let Err(e) = validate_integer_bytes(bytes) {
            return Err(format!("Invalid integer bytes: {:?}", e));
        }

        Ok(Self { bytes: bytes.to_vec() })
    }

    /// Returns the byte representation of the Integer.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }

    pub fn as_u32(&self) -> u32 {

        self.bytes.iter()
            .fold(0u32, |acc, &b| acc * 10 + (b - b'0') as u32)
    }
}