/// Checks if the given bytes represent a valid real number in PDF format.
pub fn validate_real_number_bytes(bytes: &[u8]) -> Result<(), String> {

    if bytes.is_empty() {
        return Err(format!("Real number bytes cannot be empty"));
    }

    let mut chars = bytes.iter();

    // Starts with sign
    if !bytes[0].is_ascii_digit() && bytes[0] != b'.' {

        if bytes[0] != b'+' && bytes[0] != b'-' {
            return Err(format!("Real number must start with a digit, dot, or sign: {:?}", bytes));
        }

        // Sign only is invalid
        if bytes.len() == 1 {
            return Err(format!("Real number cannot be only a sign: {:?}", bytes));
        }

        let _ = chars.next();
    }

    let mut dot_count = 0;

    let mut digit_count = 0;

    // There must be at least one digit or a dot
    for char in chars {

        // Valid characters are digits and dot
        if !char.is_ascii_digit() && char != &b'.' {
            return Err(format!("Real number contains invalid character: {:?}", bytes));
        }

        if char == &b'.' {
            dot_count += 1;
        }

        if char.is_ascii_digit() {
            digit_count += 1;
        }

        // More than one dot is invalid
        if dot_count > 1 {
            return Err(format!("Real number cannot contain more than one dot: {:?}", bytes));
        }
    }

    // There must be at least one digit
    if digit_count == 0 {
        return Err(format!("Real number must contain at least one digit: {:?}", bytes));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    
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
            assert!(super::validate_real_number_bytes(number).is_ok());
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
            assert!(super::validate_real_number_bytes(number).is_err());
        }
    }
}