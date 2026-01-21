use std::collections::HashMap;

use crate::object::direct::{Name, DirectObject};

/// PDF Dictionary entry representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DicionaryEntry {
    pub key: Name,
    pub value: DirectObject,
}

/// PDF Dictionary object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dicionary {
    entries: HashMap<Name, DirectObject>,
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

        let entries: HashMap<Name, DirectObject> = entries.into_iter().map(|e| (e.key, e.value)).collect();
        
        Self {
            entries,
            bytes,
        }
    }

    /// Returns the entries of the Dicionary.
    pub fn entries(&self) -> &HashMap<Name, DirectObject> {

        &self.entries
    }

    /// Returns the byte representation of the Dicionary.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::object::direct::{Dicionary, DicionaryEntry, Name, DirectObject, Integer};
    
    #[test]
    fn should_create_dicionary_and_return_bytes() {
        let dicionary = Dicionary::new(vec![
            DicionaryEntry {
                key: Name::new(b"/Key1").unwrap(),
                value: DirectObject::Integer(Integer::new(b"42").unwrap()),
            },
            DicionaryEntry {
                key: Name::new(b"/Key2").unwrap(),
                value: DirectObject::Integer(Integer::new(b"100").unwrap()),
            },
        ]);

        let bytes = dicionary.as_bytes();
        let expected_bytes = b"<</Key1 42 /Key2 100>>";

        assert_eq!(bytes, expected_bytes);
    }
}