/// Checks if the given byte slice represents a valid escape sequence
/// in a PDF literal string.
pub fn is_valid_escape_sequence_bytes(bytes: &[u8]) -> bool {

    // Must be between one and three bytes long
    if !matches!(bytes.len(), 1..=3) {

        return false;
    }

    // Must start with a backslash
    if bytes[0] != b'\\' {

        return false;
    }

    // If it's two bytes long, the second byte must be a valid escape character
    if bytes.len() == 2 {
        
        if !matches!(
            bytes[1],
            b'n'
            | b'r'
            | b't'
            | b'b'
            | b'f'
            | b'('
            | b')'
            | b'\r'
            | b'\n'
            | b'\\') {

            return false;
        }
    }

    // If it's not a end-of-line escape sequence, all following bytes must be digits
    if bytes != b"\\\r\n" {
        
        for byte in bytes.iter().skip(1) {
    
            if !byte.is_ascii_digit() {

                return false;
            }
        }
    }

    true
}