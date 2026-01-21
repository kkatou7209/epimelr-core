use crate::value::HexadecimalChar;

/// PDF Hexadecimal String representation (i.e `<4A6F686E>`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexadecimalString {
    /// The hexadecimal characters comprising the string.
    chars: Vec<HexadecimalChar>,
    /// The byte representation of the Hexadecimal String.
    bytes: Vec<u8>,
}

impl HexadecimalString {
    
    /// Creates a new `HexadecimalString` from a vector of `HexadecimalChar`.
    pub fn new(chars: Vec<HexadecimalChar>) -> Self {

        let capacity = chars.iter().map(|c| c.as_bytes().len()).sum::<usize>() + 2;

        let mut bytes = Vec::with_capacity(capacity);

        bytes.push(b'<');

        for hex_char in &chars {
            bytes.extend_from_slice(hex_char.as_bytes());
        }

        bytes.push(b'>');
        
        Self {
            chars,
            bytes,
        }
    }

    /// Returns the byte representation of the Hexadecimal String.
    pub fn as_bytes(&self) -> &[u8] {

        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::HexadecimalString;
    use crate::value::HexadecimalChar;

    #[test]
    fn should_returns_valid_bytes() {

        let hex_string = HexadecimalString::new(vec![
            HexadecimalChar::new(b"4A"),
            HexadecimalChar::new(b"6F"),
            HexadecimalChar::new(b"68"),
            HexadecimalChar::new(b"6E"),
        ]);

        assert_eq!(hex_string.as_bytes(), b"<4A6F686E>");
    }
}