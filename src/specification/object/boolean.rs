/// Validates if the given bytes represent a valid PDF Boolean.
/// 
/// Valid boolean bytes are "true" and "false".
pub fn validate_boolean_bytes(bytes: &[u8]) -> Result<(), String> {
    if bytes == b"true" || bytes == b"false" {
        Ok(())
    } else {
        Err(format!("Invalid boolean bytes: {:?}", bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::validate_boolean_bytes;

    #[test]
    fn should_validate_boolean_bytes() {
        let valid_bytes: &[&[u8]] = &[b"true", b"false"];

        for (index, bytes) in valid_bytes.iter().enumerate() {
            assert!(validate_boolean_bytes(bytes).is_ok(), "Expected valid: {:?} at index {}", bytes, index);
        }
    }

    #[test]
    fn should_invalidate_boolean_bytes() {
        let invalid_bytes: &[&[u8]] = &[b"True", b"FALSE", b"yes", b"no", b"1", b"0", b"", b" "];

        for (index, bytes) in invalid_bytes.iter().enumerate() {
            assert!(!validate_boolean_bytes(bytes).is_ok(), "Expected invalid: {:?} at index {}", bytes, index);
        }
    }
}