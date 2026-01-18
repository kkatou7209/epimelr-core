/// PDF Character Code representation.
#[derive(Debug, Clone)]
pub struct CharacterCode(Vec<u8>);

impl CharacterCode {
    
    /// Creates a new `CharacterCode` from the given byte vector.
    pub fn new(code: Vec<u8>) -> Self {

        

        Self(code)
    }

    /// Returns the byte representation of the Character Code.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}