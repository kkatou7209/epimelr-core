use crate::object::{Integer, Name};

/// PDF Stream Length representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Length {
    /// The Length key `Name` object
    name: Name,
    /// The Length value `Integer` object
    value: Integer,
}

impl Length {
    
    /// Creates a new `Length` object.
    pub fn new(value: Integer) -> Self {
        
        Self {
            name: Name::new(b"/Length").unwrap(),
            value
        }
    }

    /// Returns the `Name` object of the Length.
    pub fn name_object(&self) -> &Name {

        &self.name
    }

    /// Returns the value of the `Length`.
    pub fn integer_object(&self) -> &Integer {

        &self.value
    }

    /// Returns the value of the `Length` as u32.
    pub fn as_u32(&self) -> u32 {

        self.value.as_u32()
    }

    pub fn as_bytes(&self) -> Vec<u8> {

        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.value.as_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {

    use super::Length;
    use crate::object::Integer;

    #[test]
    fn should_length_returns_valid_name_and_value() {

        let length = Length::new(Integer::new(b"123").unwrap());

        assert_eq!(length.name_object().as_bytes(), b"/Length");
        assert_eq!(length.integer_object().as_bytes(), b"123");
        assert_eq!(length.as_u32(), 123);
    }
}