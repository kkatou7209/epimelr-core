/// Checks if the given bytes represent a valid byte marker value in a PDF.
/// 
/// A valid byte marker value is defined as a byte slice where each byte 
/// is greater than or equal to `0x80`.
pub fn is_valid_byte_marker_value(bytes: &[u8]) -> bool {
    !bytes.is_empty() && bytes.iter().all(|&b| b >= 0x80)
}

#[cfg(test)]
mod tests {
    use super::is_valid_byte_marker_value;

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