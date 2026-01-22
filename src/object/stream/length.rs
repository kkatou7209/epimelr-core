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
    pub fn new(len: impl Into<Integer>) -> Result<Self, String> {

        let len = len.into();

        if len.as_i32() < 0 {
            return Err("Length value must be non-negative".to_string());
        }
        
        Ok(Self {
            name: Name::new("Length").unwrap(),
            value: len,
        })
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

        self.value.as_i32() as u32
    }
}