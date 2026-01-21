use crate::object::{Boolean, Dicionary, HexadecimalString, Integer, LiteralString, Name, Null, Real};

/// A PDF Array object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    objects: Vec<ArrayElement>,
    bytes: Vec<u8>,
}

impl Array {
    
    /// Creates a new `Array` from the given objects.
    pub fn new(objects: Vec<ArrayElement>) -> Self {

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
    pub fn as_objects(&self) -> &[ArrayElement] {
        
        &self.objects
    }

    /// Returns the byte representation of the Array.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}

/// PDF Array element representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayElement {
    /// PDF `Array` object.
    Array(Array),
    /// PDF `Boolean` object.
    Boolean(Boolean),
    /// PDF `Dicionary` object.
    Dicionary(Dicionary),
    /// PDF `LiteralString` object.
    LiteralString(LiteralString),
    /// PDF `HexadecimalString` object.
    HexadecimalString(HexadecimalString),
    /// PDF `Integer` object.
    Integer(Integer),
    /// PDF `Name` object.
    Name(Name),
    /// PDF `Null` object.
    Null(Null),
    /// PDF `Real` object.
    Real(Real),
}

impl ArrayElement {
    
    /// Returns the byte representation of the Array element.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            ArrayElement::Array(obj) => obj.as_bytes(),
            ArrayElement::Boolean(obj) => obj.as_bytes(),
            ArrayElement::Dicionary(obj) => obj.as_bytes(),
            ArrayElement::Integer(obj) => obj.as_bytes(),
            ArrayElement::Name(obj) => obj.as_bytes(),
            ArrayElement::Null(obj) => obj.as_bytes(),
            ArrayElement::Real(obj) => obj.as_bytes(),
            ArrayElement::LiteralString(obj) => obj.as_bytes(),
            ArrayElement::HexadecimalString(obj) => obj.as_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::{Array, ArrayElement, Boolean, Dicionary, DicionaryEntry, DictionaryValue, HexadecimalString, Integer, LiteralString, Name, Null, Real};
    use crate::value::{HexadecimalChar, LiteralChar, Ascii, EscapeSequence};

    #[test]
    fn should_returns_valid_bytes() {

        let array = Array::new(vec![
            ArrayElement::Integer(Integer::new(b"42").unwrap()),
            ArrayElement::Boolean(Boolean::new(true)),
            ArrayElement::Name(Name::new(b"/TestName").unwrap()),
            ArrayElement::Null(Null::new()),
            ArrayElement::Real(Real::new(b"3.14").unwrap()),
            ArrayElement::LiteralString(LiteralString::new(vec![
                LiteralChar::Ascii(Ascii::new(b'A')),
                LiteralChar::Ascii(Ascii::new(b'B')),
                LiteralChar::Ascii(Ascii::new(b'C')),
                LiteralChar::EscapeSequence(EscapeSequence::Tab),
                LiteralChar::Ascii(Ascii::new(b'D')),
                LiteralChar::EscapeSequence(EscapeSequence::EndOfLine)
            ])),
            ArrayElement::Dicionary(Dicionary::new(vec![
                DicionaryEntry {
                    key: Name::new(b"/Elements").unwrap(),
                    value: DictionaryValue::Array(Array::new(vec![
                        ArrayElement::Integer(Integer::new(b"0").unwrap()),
                        ArrayElement::Real(Real::new(b"-0.4").unwrap())
                    ])),
                },
                DicionaryEntry {
                    key: Name::new(b"/Count").unwrap(),
                    value: DictionaryValue::Integer(Integer::new(b"+2").unwrap()),
                },
            ])),
            ArrayElement::HexadecimalString(HexadecimalString::new(vec![
                HexadecimalChar::new(b"4A"),
                HexadecimalChar::new(b"6F"),
                HexadecimalChar::new(b"68"),
                HexadecimalChar::new(b"6E"),
            ])),
            ArrayElement::Array(Array::new(vec![
                ArrayElement::Integer(Integer::new(b"1").unwrap()),
                ArrayElement::Integer(Integer::new(b"2").unwrap()),
                ArrayElement::Integer(Integer::new(b"3").unwrap()),
                ArrayElement::Array(Array::new(vec![
                    ArrayElement::Integer(Integer::new(b"76").unwrap()),
                    ArrayElement::LiteralString(LiteralString::new(vec![
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