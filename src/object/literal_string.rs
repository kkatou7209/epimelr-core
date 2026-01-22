use crate::value::LiteralChar;

/// PDF Literal String representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    /// The characters comprising the Literal String.
    chars: Vec<LiteralChar>,
}

impl LiteralString {
    
    /// Creates a new `LiteralString` from the given vector of `LiteralCharacter`.
    pub fn new(chars: Vec<LiteralChar>) -> Self {

        Self {
            chars,
        }
    }

    /// Returns the characters of the Literal String.
    pub fn chars(&self) -> &[LiteralChar] {
        
        &self.chars
    }
}