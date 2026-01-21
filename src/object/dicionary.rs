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
    bytes: Vec<u8>,
}

impl Dicionary {
    
    /// Creates a new `Dicionary` from the given entries.
    pub fn new(entries: Vec<DicionaryEntry>) -> Self {

        let mut bytes = Vec::new();

        bytes.extend_from_slice(b"<<");
        
        for (index, entry) in &mut entries.iter().enumerate() {

            if index > 0 && entries.len() > 1 {
                bytes.push(b' ');
            }

            bytes.extend_from_slice(entry.key.as_bytes());
            bytes.push(b' ');
            bytes.extend_from_slice(entry.value.as_bytes());
        }
        
        bytes.extend_from_slice(b">>");

        let entries: HashMap<Name, DictionaryValue> = entries.into_iter().map(|e| (e.key, e.value)).collect();
        
        Self {
            entries,
            bytes,
        }
    }

    /// Returns the entries of the Dicionary.
    pub fn entries(&self) -> &HashMap<Name, DictionaryValue> {
        &self.entries
    }

    /// Returns the byte representation of the Dicionary.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
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

impl DictionaryValue {
    
    /// Returns the byte representation of the Array element.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            DictionaryValue::Array(obj) => obj.as_bytes(),
            DictionaryValue::Boolean(obj) => obj.as_bytes(),
            DictionaryValue::Dicionary(obj) => obj.as_bytes(),
            DictionaryValue::Integer(obj) => obj.as_bytes(),
            DictionaryValue::Name(obj) => obj.as_bytes(),
            DictionaryValue::Null(obj) => obj.as_bytes(),
            DictionaryValue::Real(obj) => obj.as_bytes(),
            DictionaryValue::LiteralString(obj) => obj.as_bytes(),
            DictionaryValue::HexadecimalString(obj) => obj.as_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::{Dicionary, DicionaryEntry, Name, Integer, DictionaryValue};
    
    #[test]
    fn should_create_dicionary_and_return_bytes() {
        
        let dicionary = Dicionary::new(vec![
            DicionaryEntry {
                key: Name::new(b"/Key1").unwrap(),
                value: DictionaryValue::Integer(Integer::new(b"42").unwrap()),
            },
            DicionaryEntry {
                key: Name::new(b"/Key2").unwrap(),
                value: DictionaryValue::Integer(Integer::new(b"100").unwrap()),
            },
        ]);

        let bytes = dicionary.as_bytes();
        let expected_bytes = b"<</Key1 42 /Key2 100>>";

        assert_eq!(bytes, expected_bytes);
    }
}