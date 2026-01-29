use std::fmt::Display;

use crate::specification::object::name::validate_name_bytes;

/// PDF Name object representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    /// The bytes of the name.
    value: String,
}

impl Name {
    
    /// Creates a new `Name` from the given bytes.
    pub fn new(name: impl Into<String>) -> Self {

        let name = name.into();


        Self { value: name }
    }

    /// Returns the string value of the Name.
    pub fn as_str(&self) -> &str {
        
        &self.value
    }
}

impl Display for Name {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut str = String::new();

        for byte in self.value.as_bytes() {
            
            // Characters that must be escaped in PDF names
            if matches!(byte, b'#' | b'/' | b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'%' | 0x00..=0x20 | 0x7F..=0xFF) {
                str.push_str(&format!("#{:02X}", byte));
                continue;
            }

            str.push(*byte as char);
        }
        
        write!(f, "/{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::Name;

    #[test]
    fn should_create_valid_name() {
        let name = Name::new("ExampleName");
        assert_eq!(name.as_str(), "ExampleName");
    }

    #[test]
    fn should_format_name_correctly() {
        let name = Name::new("NameWith#Special/Chars");
        assert_eq!(format!("{}", name), "/NameWith#23Special#2FChars");
    }
}