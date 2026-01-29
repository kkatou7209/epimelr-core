mod array;
mod boolean;
mod dicionary;
mod integer;
mod name;
mod null;
mod object_stream;
mod object;
mod real;
mod reference;
mod stream;
mod string;

use std::fmt::Display;

pub use array::{Array, ArrayElement};
pub use boolean::Boolean;
pub use dicionary::{Dictionary, DictionaryEntry, DictionaryValue};
pub use string::{HexadecimalString, LiteralString};
pub use integer::Integer;
pub use name::Name;
pub use null::Null;
pub use real::Real;
pub use object::Object;
pub use reference::Reference;
pub use stream::Stream;

/// PDF Direct Object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectObject {
    /// PDF `Array` object.
    Array(Array),
    /// PDF `Boolean` object.
    Boolean(Boolean),
    /// PDF `Dicionary` object.
    Dicionary(Dictionary),
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
    /// PDF `Stream` object.
    Stream(Stream),
}

impl Display for DirectObject {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            DirectObject::Array(obj) => write!(f, "{}", obj),
            DirectObject::Boolean(obj) => write!(f, "{}", obj),
            DirectObject::Dicionary(obj) => write!(f, "{}", obj),
            DirectObject::LiteralString(obj) => write!(f, "{}", obj),
            DirectObject::HexadecimalString(obj) => write!(f, "{}", obj),
            DirectObject::Integer(obj) => write!(f, "{}", obj),
            DirectObject::Name(obj) => write!(f, "{}", obj),
            DirectObject::Null(obj) => write!(f, "{}", obj),
            DirectObject::Real(obj) => write!(f, "{}", obj),
            DirectObject::Stream(obj) => write!(f, "{}", obj),
        }
    }
}

/// PDF Indirect Object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndirectObject {
    /// PDF `Object` object.
    Object(Object),
    /// PDF `Reference` object.
    Reference(Reference),
}

impl Display for IndirectObject {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            IndirectObject::Object(obj) => write!(f, "{}", obj),
            IndirectObject::Reference(obj) => write!(f, "{}", obj),
        }
    }
}