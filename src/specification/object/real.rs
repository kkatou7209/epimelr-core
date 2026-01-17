
/// Checks if the given byte slice represents a valid real number in PDF syntax.
pub fn is_valid_real_number_bytes(bytes: &[u8]) -> bool {

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

    let mut dot_count = 0;

    let mut digit_count = 0;

    // There must be at least one digit or a dot
    for char in chars {

        // Valid characters are digits and dot
        if !char.is_ascii_digit() && char != &b'.' {
            return false;
        }

        if char == &b'.' {
            dot_count += 1;
        }

        if char.is_ascii_digit() {
            digit_count += 1;
        }

        // More than one dot is invalid
        if dot_count > 1 {
            return false;
        }
    }

    // There must be at least one digit
    if digit_count == 0 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_valid_real_number_bytes;

    #[test]
    fn should_returns_true_if_valid_real_numbers() {
        let valid_real_numbers: &[&[u8]] = &[
            b"0.0",
            b"123.456",
            b"-0.789",
            b"+3.14",
            b"42",
            b"-7",
            b".5",
            b"-.25",
            b"+.75",
        ];

        for &number in valid_real_numbers {
            assert!(is_valid_real_number_bytes(number));
        }
    }

    #[test]
    fn should_returns_false_if_invalid_real_numbers() {
        
        let invalid_real_numbers: &[&[u8]] = &[
            b"",
            b"-",
            b"+",
            b".",
            b"12.34.56",
            b"12a34",
            b" 123.45",
            b"++3.14",
            b"--2.71",
            b"3..14",
            b".-5",
            b"+-0.5",
            b"-.",
            b"+.",
        ];

        for &number in invalid_real_numbers {
            assert!(!is_valid_real_number_bytes(number));
        }
    }
}