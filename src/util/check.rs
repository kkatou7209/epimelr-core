//! Utility functions for checking PDF bytes.

/// Checks if the given byte is a valid name character in a PDF Name object.
/// 
/// A valid name character is any byte outside the range `0x21` to `0x7E` (inclusive).
pub fn is_valid_name_char(byte: &u8) -> bool {
    &0x21 <= byte && byte <= &0x7E
}

/// Checks if the given bytes represent a valid byte marker value in a PDF.
/// 
/// A valid byte marker value is defined as a byte slice where each byte 
/// is greater than or equal to `0x80`.
pub fn is_valid_byte_marker_value(bytes: &[u8]) -> bool {
    bytes.len() > 0 && bytes.iter().all(|&b| b >= 0x80)
}

#[cfg(test)]
mod tests {
    use super::is_valid_name_char;

    #[test]
    fn should_validate_name_characters() {
        
        // Valid name characters (within 0x21 to 0x7E)
        let valid_bytes = [0x21, 0x30, 0x41, 0x5A, 0x61, 0x7E];
        for &byte in &valid_bytes {
            assert!(is_valid_name_char(&byte), "Byte {:X} should be valid", byte);
        }

        // Invalid name characters (outside 0x21 to 0x7E)
        let invalid_bytes = [0x00, 0x20, 0x7F, 0x80, 0xFF];
        for &byte in &invalid_bytes {
            assert!(!is_valid_name_char(&byte), "Byte {:X} should be invalid", byte);
        }
    }

    #[test]
    fn should_validate_byte_marker_values() {
        use super::is_valid_byte_marker_value;

        // Valid byte marker values
        let valid_values = [vec![0x80], vec![0xFF], vec![0xE2, 0xE3, 0xCF, 0xD3]];
        for value in &valid_values {
            assert!(is_valid_byte_marker_value(value), "Value {:?} should be valid", value);
        }

        // Invalid byte marker values
        let invalid_values = [vec![0x7F], vec![0x00], vec![0x80, 0x7F], vec![0x00, 0xFF], vec![]];
        for value in &invalid_values {
            assert!(!is_valid_byte_marker_value(value), "Value {:?} should be invalid", value);
        }
    }
}