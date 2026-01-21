use crate::object::direct::DirectObject;

/// A PDF Array object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    objects: Vec<DirectObject>,
    bytes: Vec<u8>,
}

impl Array {
    
    /// Creates a new `Array` from the given objects.
    pub fn new(objects: Vec<DirectObject>) -> Self {

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
    pub fn as_objects(&self) -> &[DirectObject] {
        
        &self.objects
    }

    /// Returns the byte representation of the Array.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::object::direct::dicionary::DicionaryEntry;
    use crate::object::direct::{HexadecimalString, LiteralString, DirectObject};
    use crate::object::direct::integer::Integer;
    use crate::object::direct::boolean::Boolean;
    use crate::object::direct::name::Name;
    use crate::object::direct::null::Null;
    use crate::object::direct::real::Real;
    use crate::object::direct::array::Array;
    use crate::value::{HexadecimalChar, LiteralChar, Ascii, EscapeSequence};

    #[test]
    fn should_returns_valid_bytes() {

        let array = Array::new(vec![
            DirectObject::Integer(Integer::new(b"42").unwrap()),
            DirectObject::Boolean(Boolean::new(true)),
            DirectObject::Name(Name::new(b"/TestName").unwrap()),
            DirectObject::Null(Null::new()),
            DirectObject::Real(Real::new(b"3.14").unwrap()),
            DirectObject::LiteralString(LiteralString::new(vec![
                LiteralChar::Ascii(Ascii::new(b'A')),
                LiteralChar::Ascii(Ascii::new(b'B')),
                LiteralChar::Ascii(Ascii::new(b'C')),
                LiteralChar::EscapeSequence(EscapeSequence::Tab),
                LiteralChar::Ascii(Ascii::new(b'D')),
                LiteralChar::EscapeSequence(EscapeSequence::EndOfLine)
            ])),
            DirectObject::Dicionary(crate::object::direct::dicionary::Dicionary::new(vec![
                DicionaryEntry {
                    key: Name::new(b"/Elements").unwrap(),
                    value: DirectObject::Array(Array::new(vec![
                        DirectObject::Integer(Integer::new(b"0").unwrap()),
                        DirectObject::Real(Real::new(b"-0.4").unwrap())
                    ])),
                },
                DicionaryEntry {
                    key: Name::new(b"/Count").unwrap(),
                    value: DirectObject::Integer(Integer::new(b"+2").unwrap()),
                },
            ])),
            DirectObject::HexadecimalString(HexadecimalString::new(vec![
                HexadecimalChar::new(b"4A"),
                HexadecimalChar::new(b"6F"),
                HexadecimalChar::new(b"68"),
                HexadecimalChar::new(b"6E"),
            ])),
            DirectObject::Array(Array::new(vec![
                DirectObject::Integer(Integer::new(b"1").unwrap()),
                DirectObject::Integer(Integer::new(b"2").unwrap()),
                DirectObject::Integer(Integer::new(b"3").unwrap()),
                DirectObject::Array(Array::new(vec![
                    DirectObject::Integer(Integer::new(b"76").unwrap()),
                    DirectObject::LiteralString(LiteralString::new(vec![
                        LiteralChar::Ascii(Ascii::new(b'F')),
                        LiteralChar::EscapeSequence(EscapeSequence::RightParenthesis),
                        LiteralChar::EscapeSequence(EscapeSequence::CarriageReturn),
                    ]))
                ])),
            ])),
        ]);

        assert_eq!(
            array.as_bytes(),
            b"[42 true /TestName null 3.14 (ABC\\tD\n) <</Elements [0 -0.4] /Count +2>> <4A6F686E> [1 2 3 [76 (F\\)\\r)]]]"
        );
    }
}