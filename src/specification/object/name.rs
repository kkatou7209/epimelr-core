/// Checks if the given byte is a valid name character in a PDF Name object.
/// 
/// A valid name character is any byte outside the range `0x21` to `0x7E` (inclusive).
pub fn validate_name_char(byte: &u8) -> Result<(), String> {
    
    if &0x21 <= byte && byte <= &0x7E {

        return Ok(());
    }

    Err(format!("Invalid name character: {}", *byte as char))
}

/// Validates if the given byte slice is a valid PDF Name representation.
pub fn validate_name_bytes(bytes: &[u8]) -> Result<(), String> {
    
    // empty bytes
    if bytes.is_empty() {
        return Err(format!("Name bytes cannot be empty"));
    }

    // starts with other than solidus
    if bytes[0] != b'/' {
        return Err(format!("Name must start with a solidus"));
    }

    for &byte in bytes.iter().skip(1) {

        // contains invalid name character
        if let Err(e) = validate_name_char(&byte) {
            return Err(e);
        }

        // contains solidus internally
        if &byte == &b'/' {
            return Err(format!("Name cannot contain solidus internally"));
        }
    }

    // ends with space
    if bytes[bytes.len() - 1] == b' ' {
        return Err(format!("Name cannot end with a space"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_validate_name_characters() {
        
        // Valid name characters (within 0x21 to 0x7E)
        let valid_bytes: &[u8] = &[0x21, 0x30, 0x41, 0x5A, 0x61, 0x7E];
        for &byte in valid_bytes {
            assert!(super::validate_name_char(&byte).is_ok(), "Byte {:X} should be valid", byte);
        }

        // Invalid name characters (outside 0x21 to 0x7E)
        let invalid_bytes: &[u8] = &[0x00, 0x20, 0x7F, 0x80, 0xFF];
        for &byte in invalid_bytes {
            assert!(super::validate_name_char(&byte).is_err(), "Byte {:X} should be invalid", byte);
        }
    }

    #[test]
    fn should_validate_name_bytes() {
        
        // Valid name bytes
        let valid_names: &[&[u8]] = &[b"/ValidName", b"/Another_Valid-Name123", b"/NameWithSymbols!@#"];
        for &name in valid_names {
            assert!(super::validate_name_bytes(name).is_ok(), "Name {:?} should be valid", name);
        }

        // Invalid name bytes
        let invalid_names: &[&[u8]] = &[b"", b"InvalidName", b"/Invalid Name ", b"/Invalid/Name", b"/Invalid\x7FName"];
        for &name in invalid_names {
            assert!(super::validate_name_bytes(name).is_err(), "Name {:?} should be invalid", name);
        }
    }
}