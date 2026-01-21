use crate::object::{Array, Boolean, Dicionary, HexadecimalString, Integer, LiteralString, Name, Null, Real};

/// PDF Indirect Object representation.
#[derive(Debug, Clone, Eq)]
pub struct Object {
    /// PDF Indirect Object number.
    number: u32,
    /// PDF Indirect Object generation.
    generation: u32,
    /// PDF Indirect Object value.
    object: DictionaryValue,
}

impl Object {
    
    /// Creates a new `IndirectObject` from the given number, generation, and object.
    pub fn new(number: u32, generation: u32, object: DictionaryValue) -> Self {
        
        Self {
            number,
            generation,
            object,
        }
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
    pub fn object(&self) -> &DictionaryValue {
        
        &self.object
    }

    /// Returns the byte representation of the Indirect Object.
    pub fn as_bytes(&self) -> Vec<u8> {

        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.number.to_string().as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.generation.to_string().as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(b"obj\n");
        bytes.extend_from_slice(self.object.as_bytes());
        bytes.push(b'\n');
        bytes.extend_from_slice(b"endobj");

        bytes
    }
}

impl PartialEq for Object {
    
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number 
        && self.generation == other.generation
    }
}

#[cfg(test)]
mod tests {

    use crate::object::Null;
    use crate::object::object::DictionaryValue;

    use super::Object;

    #[test]
    fn should_equal_when_number_and_generation_are_equal() {
        let obj1 = Object::new(1, 0, DictionaryValue::Null(Null::new()));
        let obj2 = Object::new(1, 0, DictionaryValue::Null(Null::new()));
        assert_eq!(obj1, obj2);
    }

    #[test]
    fn should_not_equal_when_number_or_generation_are_different() {
        let obj1 = Object::new(1, 0, DictionaryValue::Null(Null::new()));
        let obj2 = Object::new(2, 0, DictionaryValue::Null(Null::new()));
        let obj3 = Object::new(1, 1, DictionaryValue::Null(Null::new()));
        assert_ne!(obj1, obj2);
        assert_ne!(obj1, obj3);
    }

    #[test]
    fn should_return_valid_bytes() {
        let indirect_object = Object::new(1, 0, DictionaryValue::Null(Null::new()));
        let expected_bytes = b"1 0 obj\nnull\nendobj".to_vec();
        assert_eq!(indirect_object.as_bytes(), expected_bytes);
    }
}