use crate::specification::object::integer::is_valid_integer_bytes;

/// PDF Integer object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    bytes: Vec<u8>,
}

impl Integer {

    /// Creates a new `Integer` from the given bytes.
    pub fn new(bytes: &[u8]) -> Self {

        if !is_valid_integer_bytes(bytes) {
            panic!("integer contains invalid characters");
        }

        Self { bytes: bytes.to_vec() }
    }

    /// Returns the byte representation of the Integer.
    pub fn as_bytes(&self) -> &[u8] {

        if !is_valid_integer_bytes(&self.bytes) {
            panic!("integer contains invalid characters");
        }

        &self.bytes
    }
}