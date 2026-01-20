use crate::value::LiteralChar;

/// PDF Literal String representation.
#[derive(Debug, Clone)]
pub struct LiteralString {
    chars: Vec<LiteralChar>,
    bytes: Vec<u8>,
}

impl LiteralString {
    
    /// Creates a new `LiteralString` from the given vector of `LiteralCharacter`.
    pub fn new(chars: Vec<LiteralChar>) -> Self {

        let capacity = chars.iter()
            .map(|c| c.as_bytes().len())
            .sum::<usize>()
            + 2 + chars.len();

        let mut bytes = Vec::with_capacity(capacity);

        bytes.push(b'(');
        
        for (index, character) in chars.iter().enumerate() {

            bytes.extend_from_slice(character.as_bytes());
        }

        bytes.push(b')');

        Self {
            chars,
            bytes,
        }
    }

    /// Returns the characters of the Literal String.
    pub fn characters(&self) -> &[LiteralChar] {
        
        &self.chars
    }

    /// Returns the byte representation of the Literal String.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}