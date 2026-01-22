use crate::specification::value::literal_char::escape_sequence::{validate_escape_sequence_bytes, validate_escaped_char_code};

/// PDF escaped character code representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterCode(u8);

impl CharacterCode {
    
    /// Creates a new `CharacterCode` from the given byte vector.
    pub fn new(code: u8) -> Self {

        Self(code)
    }

    pub fn as_byte(&self) -> u8 {
        
        self.0
    }
}