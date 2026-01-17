//! This module contains PDF object representations.
mod array;
mod boolean;
mod integer;
mod name;
mod null;
mod real;

use crate::object::array::Array;
use crate::object::boolean::Boolean;
use crate::object::integer::Integer;
use crate::object::name::Name;
use crate::object::null::Null;
use crate::object::real::Real;

/// PDF Object representation.
#[derive(Debug, Clone)]
pub enum Object {
    /// PDF `Array` object.
    Array(Array),
    /// PDF `Boolean` object.
    Boolean(Boolean),
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
        }
    }
}