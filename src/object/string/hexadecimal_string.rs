use std::fmt::Display;

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

impl Display for HexadecimalString {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "<")?;
        
        for char in &self.chars {
            write!(f, "{}", char)?;
        }
        
        write!(f, ">")
    }
}

/// PDF Hexadecimal string character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalChar(u8);

impl HexadecimalChar {
    
    /// Creates a new `HexadecimalChar` character sequence from a byte vector.
    pub fn new(char: u8) -> Self {

        Self(char)
    }

    /// Returns the character byte.
    pub fn as_byte(&self) -> u8 {

        self.0
    }
}

impl Display for HexadecimalChar {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "{:02X}", self.0)
    }
}