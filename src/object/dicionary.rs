use std::collections::HashMap;

use crate::object::*;

/// PDF Dictionary entry representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DicionaryEntry {
    pub key: Name,
    pub value: DictionaryValue,
}

/// PDF Dictionary object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dicionary {
    entries: HashMap<Name, DictionaryValue>,
}

impl Dicionary {
    
    /// Creates a new `Dicionary` from the given entries.
    pub fn new(entries: Vec<DicionaryEntry>) -> Self {

        Self {
            entries: entries.into_iter().map(|e| (e.key, e.value)).collect(),
        }
    }

    /// Returns the entries of the Dicionary.
    pub fn entries(&self) -> &HashMap<Name, DictionaryValue> {
        &self.entries
    }
}

/// PDF Array element representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DictionaryValue {
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