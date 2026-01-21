//! This module contains PDF object representations.

/// Submodules for direct objects.
mod array;
mod boolean;
mod dicionary;
mod hexadecimal_string;
mod integer;
mod literal_string;
mod name;
mod null;
mod real;

pub use array::Array;
pub use boolean::Boolean;
pub use dicionary::{Dicionary, DicionaryEntry};
pub use hexadecimal_string::HexadecimalString;
pub use integer::Integer;
pub use literal_string::LiteralString;
pub use name::Name;
pub use null::Null;
pub use real::Real;

/// PDF Object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectObject {
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

impl DirectObject {
    
    /// Returns the byte representation of the Object.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            DirectObject::Array(obj) => obj.as_bytes(),
            DirectObject::Boolean(obj) => obj.as_bytes(),
            DirectObject::Dicionary(obj) => obj.as_bytes(),
            DirectObject::Integer(obj) => obj.as_bytes(),
            DirectObject::Name(obj) => obj.as_bytes(),
            DirectObject::Null(obj) => obj.as_bytes(),
            DirectObject::Real(obj) => obj.as_bytes(),
            DirectObject::LiteralString(obj) => obj.as_bytes(),
            DirectObject::HexadecimalString(obj) => obj.as_bytes(),
        }
    }
}