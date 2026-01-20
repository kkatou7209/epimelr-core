use crate::specification::object::name::validate_name_bytes;

/// PDF Name object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    /// The bytes of the name.
    bytes: Vec<u8>,
}

impl Name {
    
    /// Creates a new `Name` from the given bytes.
    pub fn new(bytes: &[u8]) -> Result<Self, String> {
        
        if let Err(e) = validate_name_bytes(bytes) {
            return Err(format!("Invalid name bytes: {:?}", e));
        }

        Ok(Self { bytes: bytes.to_vec() })
    }

    /// Returns the byte representation of the Name.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn should_create_valid_name() {
        let name = Name::new(b"/ExampleName").unwrap();
        assert_eq!(name.as_bytes(), b"/ExampleName");
    }

    #[test]
    fn should_error_when_creating_name_with_invalid_characters() {
        let result = Name::new(b"/Invalid Name ");
        assert!(result.is_err());
    }
}