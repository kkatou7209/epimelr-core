use crate::value::character::ascii::Ascii;

mod ascii;
mod escape_sequence;

/// PDF Character representation.
#[derive(Debug, Clone)]
pub enum Character {
    /// PDF ASCII character representation.
    Ascii(Ascii),
}