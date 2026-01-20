/// Validates if the given byte slice is a valid integer 
/// representation according to PDF specification.
pub fn validate_integer_bytes(bytes: &[u8]) -> Result<(), String> {
    
    if bytes.is_empty() {
        return Err("Integer bytes cannot be empty".to_string());
    }

    if !bytes[0].is_ascii_digit() {
        
        if bytes[0] != b'+' && bytes[0] != b'-' {
            return Err(format!("Invalid starting character for integer: {}", bytes[0] as char));
        }

        if bytes.len() == 1 {
            return Err("Integer cannot be only a sign".to_string());
        }

        for &byte in &bytes[1..] {

            if !byte.is_ascii_digit() {
                return Err(format!("Invalid character in integer: {}", byte as char));
            }
        }

        return Ok(());
    }

    for &byte in bytes {

        if !byte.is_ascii_digit() {
            return Err(format!("Invalid character in integer: {}", byte as char));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::specification::object::integer::validate_integer_bytes;

    
    #[test]
    fn should_validate_integer_bytes() {

        let valid_bytes: &[&[u8]] = &[
            b"0",
            b"123",
            b"-456",
            b"+789",
            b"00123",
        ];

        for (index, bytes) in valid_bytes.iter().enumerate() {
            assert!(validate_integer_bytes(bytes).is_ok(), "Expected valid: {:?} at index {}", bytes, index);
        }
    }

    #[test]
    fn should_invalidate_integer_bytes() {

        let invalid_bytes: &[&[u8]] = &[
            b"",
            b"-",
            b"+",
            b"12a3",
            b"++123",
            b"--456",
            b"12.34",
            b" 123",
            b"123 ",
        ];

        for (index, bytes) in invalid_bytes.iter().enumerate() {
            assert!(!validate_integer_bytes(bytes).is_ok(), "Expected invalid: {:?} at index {}", bytes, index);
        }
    }
}