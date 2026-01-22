use crate::specification::value::hexadecimal_char::validate_hexadecimal_char;

/// PDF Hexadecimal string character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalChar(char);

impl HexadecimalChar {
    
    /// Creates a new `HexadecimalChar` character sequence from a byte vector.
    pub fn new(char: char) -> Self {
        
        Self(char)
    }
}