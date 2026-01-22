mod ascii;
mod escape_sequence;

pub use ascii::Ascii;
pub use escape_sequence::EscapeSequence;

/// PDF Literal string character representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralChar {
    /// PDF ASCII character representation.
    Ascii(Ascii),
    /// PDF Escape Sequence character representation.
    EscapeSequence(EscapeSequence),
}