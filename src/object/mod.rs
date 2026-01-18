//! This module contains PDF object representations.
mod array;
mod boolean;
mod hexadecimal_string;
mod integer;
mod literal_string;
mod name;
mod null;
mod real;

pub use array::Array;
pub use boolean::Boolean;
pub use hexadecimal_string::HexadecimalString;
pub use integer::Integer;
pub use literal_string::LiteralString;
pub use name::Name;
pub use null::Null;
pub use real::Real;

/// PDF Object representation.
#[derive(Debug, Clone)]
pub enum Object {
    /// PDF `Array` object.
    Array(Array),
    /// PDF `Boolean` object.
    Boolean(Boolean),
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

impl Object {
    
    /// Returns the byte representation of the Object.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Object::Array(obj) => obj.as_bytes(),
            Object::Boolean(obj) => obj.as_bytes(),
            Object::Integer(obj) => obj.as_bytes(),
            Object::Name(obj) => obj.as_bytes(),
            Object::Null(obj) => obj.as_bytes(),
            Object::Real(obj) => obj.as_bytes(),
            _ => unimplemented!(),
        }
    }
}