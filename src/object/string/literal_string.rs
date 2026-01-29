use std::fmt::Display;

use crate::object::string::{Ascii, EscapeSequence};

/// PDF Literal String representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiteralString {
    /// The characters comprising the Literal String.
    chars: Vec<LiteralChar>,
}

impl LiteralString {
    
    /// Creates a new `LiteralString` from the given vector of `LiteralCharacter`.
    pub fn new(chars: impl IntoIterator<Item = LiteralChar>) -> Self {

        Self {
            chars: chars.into_iter().collect(),
        }
    }

    /// Returns the characters of the Literal String.
    pub fn chars(&self) -> &[LiteralChar] {
        
        &self.chars
    }
}

impl Display for LiteralString {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "(")?;

        for char in &self.chars {
            write!(f, "{}", char)?;
        }

        write!(f, ")")
    }
}

/// PDF Literal string character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralChar {
    /// PDF ASCII character representation.
    Ascii(Ascii),
    /// PDF Escape Sequence character representation.
    EscapeSequence(EscapeSequence),
}

impl Display for LiteralChar {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            LiteralChar::Ascii(ascii) => write!(f, "{}", ascii),
            LiteralChar::EscapeSequence(escape) => write!(f, "{}", escape),
        }
    }
}