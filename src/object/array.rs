use std::fmt::Display;

use crate::object::{Boolean, Dicionary, HexadecimalString, Integer, LiteralString, Name, Null, Real};

/// A PDF Array object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    objects: Vec<ArrayElement>,
}

impl Array {
    
    /// Creates a new `Array` from the given objects.
    pub fn new(objects: impl IntoIterator<Item = ArrayElement>) -> Self {

        Self { objects: objects.into_iter().collect() }
    }

    /// Returns the objects contained in the Array.
    pub fn as_objects(&self) -> &[ArrayElement] {
        
        &self.objects
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "[")?;

        for (i, obj) in self.objects.iter().enumerate() {
        
            if i > 0 {
                write!(f, " ")?;
            }

            write!(f, "{}", obj)?;
        }
        
        write!(f, "]")
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

impl Display for ArrayElement {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            ArrayElement::Array(obj) => write!(f, "{}", obj),
            ArrayElement::Boolean(obj) => write!(f, "{}", obj),
            ArrayElement::Dicionary(obj) => write!(f, "{}", obj),
            ArrayElement::LiteralString(obj) => write!(f, "{}", obj),
            ArrayElement::HexadecimalString(obj) => write!(f, "{}", obj),
            ArrayElement::Integer(obj) => write!(f, "{}", obj),
            ArrayElement::Name(obj) => write!(f, "{}", obj),
            ArrayElement::Null(obj) => write!(f, "{}", obj),
            ArrayElement::Real(obj) => write!(f, "{}", obj),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::string::HexadecimalChar;
    use crate::object::{Array, ArrayElement, Boolean, HexadecimalString, Integer, Name, Null, Real};


    #[test]
    fn should_format_array_correctly() {

        let array = Array::new(vec![
            ArrayElement::Integer(Integer::new("42").unwrap()),
            ArrayElement::Name(Name::new("Example").unwrap()),
            ArrayElement::Null(Null::new()),
            ArrayElement::Array(Array::new(vec![
                ArrayElement::Boolean(Boolean::truethy()),
                ArrayElement::Real(Real::new("2").unwrap()),
                ArrayElement::HexadecimalString(HexadecimalString::new(vec![
                    HexadecimalChar::new(0xDE),
                    HexadecimalChar::new(0xAD),
                    HexadecimalChar::new(0xBE),
                    HexadecimalChar::new(0xEF),
                ])),
            ]))
        ]);

        let formatted = format!("{}", array);

        assert_eq!(formatted, "[42 /Example null [true 2 <DEADBEEF>]]");
    }
}