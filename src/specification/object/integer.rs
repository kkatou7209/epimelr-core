/// Checks if the given byte slice is a valid integer representation according to PDF specification.
pub fn is_valid_integer_bytes(bytes: &[u8]) -> bool {

    if bytes.is_empty() {
        return false;
    }

    let mut chars = bytes.iter();

    // Starts with sign
    if bytes[0] == b'+' || bytes[0] == b'-' {

        // Sign only is invalid
        if bytes.len() == 1 {
            return false;
        }

        let _ = chars.next();
    }

    // Check remaining characters that they are all digits
    for char in chars {
        if !char.is_ascii_digit() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_integer_bytes;

    #[test]
    fn should_returns_true_if_valid_integers() {
        let valid_integers: &[&[u8]] = &[
            b"0",
            b"123",
            b"-456",
            b"+789",
        ];

        for integer in valid_integers {
            assert!(is_valid_integer_bytes(integer), "Expected {:?} to be valid", integer);
        }
    }

    #[test]
    fn should_returns_false_if_invalid_integers() {
        let invalid_integers: &[&[u8]] = &[
            b"",
            b"-",
            b"+",
            b"12a3",
            b" 123",
        ];

        for integer in invalid_integers {
            assert!(!is_valid_integer_bytes(integer), "Expected {:?} to be invalid", integer);
        }
    }
}