use std::collections::HashMap;
use std::fmt::Display;

use crate::object::*;

/// PDF Dictionary entry representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DictionaryEntry {
    pub key: Name,
    pub value: DictionaryValue,
}

/// PDF Dictionary object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dicionary {
    /// The entries of the dictionary.
    entries: HashMap<Name, DictionaryValue>,
}

impl Dicionary {
    
    /// Creates a new `Dicionary` from the given entries.
    pub fn new(entries: impl IntoIterator<Item = DictionaryEntry>) -> Self {

        Self {
            entries: entries.into_iter().map(|e| (e.key, e.value)).collect(),
        }
    }

    /// Returns whether the dictionary has the related key.
    pub fn has(&self, key: &Name) -> bool {

        self.entries.contains_key(key)
    }

    /// Returns value of related key.
    pub fn get(&self, key: &Name) -> Option<&DictionaryValue> {
        
        self.entries.get(key)
    }
}

impl Display for Dicionary {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "<<")?;
        
        for (key, value) in &self.entries {
            write!(f, "{} {:?}", key, value)?;
        }
        
        write!(f, ">>")
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
    /// PDF `Reference` object.
    Reference(Reference),
}

impl Display for DictionaryValue {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            DictionaryValue::Array(obj) => write!(f, "{}", obj),
            DictionaryValue::Boolean(obj) => write!(f, "{}", obj),
            DictionaryValue::Dicionary(obj) => write!(f, "{}", obj),
            DictionaryValue::LiteralString(obj) => write!(f, "{}", obj),
            DictionaryValue::HexadecimalString(obj) => write!(f, "{}", obj),
            DictionaryValue::Integer(obj) => write!(f, "{}", obj),
            DictionaryValue::Name(obj) => write!(f, "{}", obj),
            DictionaryValue::Null(obj) => write!(f, "{}", obj),
            DictionaryValue::Real(obj) => write!(f, "{}", obj),
            DictionaryValue::Reference(obj) => write!(f, "{}", obj),
        }
    }
}