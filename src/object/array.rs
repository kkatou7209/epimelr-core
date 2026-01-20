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
            if i > 0 && objects.len() > 1 {
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
    use crate::object::{HexadecimalString, LiteralString, Object};
    use crate::object::integer::Integer;
    use crate::object::boolean::Boolean;
    use crate::object::name::Name;
    use crate::object::null::Null;
    use crate::object::real::Real;
    use crate::object::array::Array;
    use crate::value::{HexadecimalChar, LiteralChar, Ascii, EscapeSequence};

    #[test]
    fn should_returns_valid_bytes() {

        let array = Array::new(vec![
            Object::Integer(Integer::new(b"42").unwrap()),
            Object::Boolean(Boolean::new(true)),
            Object::Name(Name::new(b"/TestName").unwrap()),
            Object::Null(Null::new()),
            Object::Real(Real::new(b"3.14").unwrap()),
            Object::LiteralString(LiteralString::new(vec![
                LiteralChar::Ascii(Ascii::new(b'A')),
                LiteralChar::Ascii(Ascii::new(b'B')),
                LiteralChar::Ascii(Ascii::new(b'C')),
                LiteralChar::EscapeSequence(EscapeSequence::Tab),
                LiteralChar::Ascii(Ascii::new(b'D')),
                LiteralChar::EscapeSequence(EscapeSequence::EndOfLineCR)
            ])),
            Object::HexadecimalString(HexadecimalString::new(vec![
                HexadecimalChar::new(b"4A"),
                HexadecimalChar::new(b"6F"),
                HexadecimalChar::new(b"68"),
                HexadecimalChar::new(b"6E"),
            ])),
            Object::Array(Array::new(vec![
                Object::Integer(Integer::new(b"1").unwrap()),
                Object::Integer(Integer::new(b"2").unwrap()),
                Object::Integer(Integer::new(b"3").unwrap()),
                Object::Array(Array::new(vec![
                    Object::Integer(Integer::new(b"76").unwrap()),
                    Object::LiteralString(LiteralString::new(vec![
                        LiteralChar::Ascii(Ascii::new(b'F')),
                        LiteralChar::EscapeSequence(EscapeSequence::RightParenthesis),
                        LiteralChar::EscapeSequence(EscapeSequence::CarriageReturn),
                    ]))
                ])),
            ])),
        ]);

        assert_eq!(
            array.as_bytes(),
            b"[42 true /TestName null 3.14 (ABC\\tD\r) <4A6F686E> [1 2 3 [76 (F\\)\\r)]]]"
        );
    }
}