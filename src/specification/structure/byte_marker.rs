/// Validates if the given byte slice is a valid byte marker value
/// according to PDF specification.
pub fn validate_byte_marker_value(bytes: &[u8]) -> Result<(), String> {
    
    if bytes.is_empty() {
        return Err("Byte marker value cannot be empty".to_string());
    }

    if !bytes.iter().all(|&b| b >= 0x80) {
        return Err("Byte marker value must consist of bytes in the range 0x80 to 0xFF.".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::specification::structure::byte_marker::validate_byte_marker_value;

    #[test]
    fn should_validate_byte_marker_values() {

        // Valid byte marker values
        let valid_values = [vec![0x80], vec![0xFF], vec![0xE2, 0xE3, 0xCF, 0xD3]];
        for value in &valid_values {
            assert!(validate_byte_marker_value(value).is_ok(), "Value {:?} should be valid", value);
        }

        // Invalid byte marker values
        let invalid_values = [vec![0x7F], vec![0x00], vec![0x80, 0x7F], vec![0x00, 0xFF], vec![]];
        for value in &invalid_values {
            assert!(validate_byte_marker_value(value).is_err(), "Value {:?} should be invalid", value);
        }
    }
}