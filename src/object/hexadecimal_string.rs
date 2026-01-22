use crate::value::HexadecimalChar;

/// PDF Hexadecimal String representation (i.e `<4A6F686E>`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalString {
    /// The hexadecimal characters comprising the string.
    chars: Vec<HexadecimalChar>,
}

impl HexadecimalString {
    
    /// Creates a new `HexadecimalString` from a vector of `HexadecimalChar`.
    pub fn new(chars: Vec<HexadecimalChar>) -> Self {
        
        Self {
            chars,
        }
    }

    /// Returns the characters of the Hexadecimal String.
    pub fn chars(&self) -> &[HexadecimalChar] {
        
        &self.chars
    }
}