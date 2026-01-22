use crate::specification::object::name::validate_name_bytes;

/// PDF Name object representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    /// The bytes of the name.
    value: String,
}

impl Name {
    
    /// Creates a new `Name` from the given bytes.
    pub fn new(name: impl AsRef<str>) -> Result<Self, String> {

        let name = name.as_ref().to_string();
        
        if name.is_empty() {
            return Err(format!("Name cannot be empty"));
        }

        Ok(Self { value: name })
    }

    /// Returns the string value of the Name.
    pub fn as_str(&self) -> &str {
        
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn should_create_valid_name() {
        let name = Name::new("ExampleName").unwrap();
        assert_eq!(name.as_str(), "ExampleName");
    }

    #[test]
    fn should_error_when_creating_name_with_invalid_characters() {
        let result = Name::new("");
        assert!(result.is_err());
    }
}