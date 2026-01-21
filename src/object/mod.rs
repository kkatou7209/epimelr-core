mod array;
mod boolean;
mod dicionary;
mod hexadecimal_string;
mod integer;
mod literal_string;
mod name;
mod null;
mod object;
mod real;
mod reference;
mod stream;

pub use array::{Array, ArrayElement};
pub use boolean::Boolean;
pub use dicionary::{Dicionary, DicionaryEntry, DictionaryValue};
pub use hexadecimal_string::HexadecimalString;
pub use integer::Integer;
pub use literal_string::LiteralString;
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
    
    /// Returns the byte representation of the Direct Object.
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

/// PDF Indirect Object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndirectObject {
    /// PDF `Object` object.
    Object(Object),
    /// PDF `Reference` object.
    Reference(Reference),
    /// PDF `Stream` object.
    Stream(Stream),
}

impl IndirectObject {
    
    /// Returns the byte representation of the Indirect Object.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            IndirectObject::Object(obj) => obj.as_bytes(),
            IndirectObject::Reference(obj) => obj.as_bytes(),
            _ => unimplemented!(),
        }
    }
}