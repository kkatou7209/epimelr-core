use std::fmt::Display;

use crate::object::{DirectObject};

/// PDF Indirect Object representation.
#[derive(Debug, Clone, Eq)]
pub struct Object {
    /// PDF Indirect Object number.
    number: u32,
    /// PDF Indirect Object generation.
    generation: u32,
    /// PDF Direct Object value.
    object: DirectObject,
}

impl Object {
    
    /// Creates a new `IndirectObject` from the given number, generation, and object.
    pub fn new(number: u32, generation: u32, object: DirectObject) -> Result<Self, String> {
        
        if number == 0 {
            return Err(format!("Object number must be greater than 0"));
        }

        Ok(Self {
            number,
            generation,
            object,
        })
    }

    /// Returns the number of the Indirect Object.
    pub fn number(&self) -> &u32 {
        
        &self.number
    }

    /// Returns the generation of the Indirect Object.
    pub fn generation(&self) -> &u32 {
        
        &self.generation
    }

    /// Returns the object of the Indirect Object.
    pub fn object(&self) -> &DirectObject {
        
        &self.object
    }
}

impl PartialEq for Object {
    
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number 
        && self.generation == other.generation
    }
}

impl Display for Object {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} obj\n{}\nendobj", self.number, self.generation, self.object)
    }
}

#[cfg(test)]
mod tests {

    use crate::object::{DirectObject, Null};

    use super::Object;

    #[test]
    fn should_equal_when_number_and_generation_are_equal() {
        let obj1 = Object::new(1, 0, DirectObject::Null(Null::new()));
        let obj2 = Object::new(1, 0, DirectObject::Null(Null::new()));
        assert_eq!(obj1, obj2);
    }

    #[test]
    fn should_not_equal_when_number_or_generation_are_different() {
        let obj1 = Object::new(1, 0, DirectObject::Null(Null::new()));
        let obj2 = Object::new(2, 0, DirectObject::Null(Null::new()));
        let obj3 = Object::new(1, 1, DirectObject::Null(Null::new()));
        assert_ne!(obj1, obj2);
        assert_ne!(obj1, obj3);
    }
}