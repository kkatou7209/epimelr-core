use crate::object::{Boolean, Dicionary, HexadecimalString, Integer, LiteralString, Name, Null, Real};

/// A PDF Array object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array {
    objects: Vec<ArrayElement>,
}

impl Array {
    
    /// Creates a new `Array` from the given objects.
    pub fn new(objects: Vec<ArrayElement>) -> Self {

        Self { objects }
    }

    /// Returns the objects contained in the Array.
    pub fn as_objects(&self) -> &[ArrayElement] {
        
        &self.objects
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