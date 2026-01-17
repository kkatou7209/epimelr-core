/// Checks if the given bytes represent a valid PDF Boolean.
///
/// Valid boolean bytes are "true" and "false".
pub fn is_valid_boolean_bytes(bytes: &[u8]) -> bool {
    bytes == b"true" || bytes == b"false"
}

#[cfg(test)]
mod tests {
    use super::is_valid_boolean_bytes;

    #[test]
    fn should_validate_boolean_bytes() {
        let valid_bytes: &[&[u8]] = &[b"true", b"false"];
        for &bytes in valid_bytes {
            assert!(is_valid_boolean_bytes(bytes), "Bytes {:?} should be valid", bytes);
        }

        let invalid_bytes: &[&[u8]] = &[b"True", b"FALSE", b"yes", b"no", b"1", b"0"];
        for &bytes in invalid_bytes {
            assert!(!is_valid_boolean_bytes(bytes), "Bytes {:?} should be invalid", bytes);
        }
    }
}