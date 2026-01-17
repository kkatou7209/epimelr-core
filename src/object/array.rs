use crate::object::Object;

/// A PDF Array object.
#[derive(Debug, Clone)]
pub struct Array {
    objects: Vec<Object>,
    bytes: Vec<u8>,
}

impl Array {
    
    /// Creates a new `Array` from the given objects.
    pub fn new(objects: Vec<Object>) -> Self {

        let mut bytes = Vec::new();
        
        bytes.push(b'[');
        
        for (i, obj) in objects.iter().enumerate() {
            if i > 0 {
                bytes.push(b' ');
            }
            bytes.extend_from_slice(&obj.as_bytes());
        }
        
        bytes.push(b']');

        Self { objects, bytes }
    }

    /// Returns the objects contained in the Array.
    pub fn as_objects(&self) -> &[Object] {
        
        &self.objects
    }

    /// Returns the byte representation of the Array.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::object::Object;
    use crate::object::integer::Integer;
    use crate::object::boolean::Boolean;
    use crate::object::name::Name;
    use crate::object::null::Null;
    use crate::object::real::Real;
    use crate::object::array::Array;

    #[test]
    fn should_returns_valid_bytes() {

        let array = Array::new(vec![
            Object::Integer(Integer::new(b"42")),
            Object::Boolean(Boolean::new(true)),
            Object::Name(Name::new(b"/TestName")),
            Object::Null(Null::new()),
            Object::Real(Real::new(b"3.14")),
            Object::Array(Array::new(vec![
                Object::Integer(Integer::new(b"1")),
                Object::Integer(Integer::new(b"2")),
                Object::Integer(Integer::new(b"3")),
            ])),
        ]);

        assert_eq!(array.as_bytes(), b"[42 true /TestName null 3.14 [1 2 3]]");
    }
}