use std::fmt::Display;

/// A PDF ASCII character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ascii(u8);

impl Ascii {
    
    /// Creates a new `Ascii` character sequence from a byte vector.
    pub fn new(char: u8) -> Result<Self, String> {

        if char.is_ascii() == false {
            return Err(format!("The provided byte {} is not an ASCII character.", char));
        }

        Ok(Self(char))
    }

    /// Returns the character byte.
    pub fn as_byte(&self) -> u8 {

        self.0
    }
}

impl Display for Ascii {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "{}", self.0 as char)
    }
}