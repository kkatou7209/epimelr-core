/// A PDF ASCII character representation.
#[derive(Debug, Clone)]
pub struct Ascii(Vec<u8>);

impl Ascii {
    
    /// Creates a new `Ascii` character sequence from a byte vector.
    pub fn new(char: u8) -> Self {

        if char.is_ascii() == false {
            panic!("The provided byte is not an ASCII character.");
        }

        Self(vec![char])
    }

    /// Returns the byte representation of the ASCII character sequence.
    pub fn as_bytes(&self) -> &[u8] {

        &self.0
    }
}