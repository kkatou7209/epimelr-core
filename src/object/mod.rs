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