mod ascii;
mod escape_sequence;

pub use ascii::Ascii;
pub use escape_sequence::EscapeSequence;

/// PDF Literal string character representation.
#[derive(Debug, Clone)]
pub enum LiteralChar {
    /// PDF ASCII character representation.
    Ascii(Ascii),
    /// PDF Escape Sequence character representation.
    EscapeSequence(EscapeSequence),
}

impl LiteralChar {
    
    /// Returns the byte representation of the Literal Character.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            LiteralChar::Ascii(ascii) => ascii.as_bytes(),
            LiteralChar::EscapeSequence(escape_sequence) => escape_sequence.as_bytes(),
        }
    }
}