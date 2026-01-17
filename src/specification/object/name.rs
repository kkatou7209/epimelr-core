/// Checks if the given byte is a valid name character in a PDF Name object.
/// 
/// A valid name character is any byte outside the range `0x21` to `0x7E` (inclusive).
pub fn is_valid_name_char(byte: &u8) -> bool {
    &0x21 <= byte && byte <= &0x7E
}

/// Checks if the given bytes represent a valid PDF Name.
/// 
/// A valid PDF Name must start with a solidus (`/`), the tail must not be a space.
pub fn is_valid_name_bytes(bytes: &[u8]) -> bool {

    // empty bytes
    if bytes.is_empty() {
        return false;
    }

    // starts with other than solidus
    if bytes[0] != b'/' {
        return false;
    }

    for &byte in bytes.iter().skip(1) {

        // contains invalid name character
        if !is_valid_name_char(&byte) {
            return false;
        }

        // contains solidus internally
        if &byte == &b'/' {
            return false;
        }
    }

    // ends with space
    if bytes[bytes.len() - 1] == b' ' {
        return false;
    }
    
    true
}

#[cfg(test)]
mod tests {
    use crate::specification::object::name::is_valid_name_char;

    #[test]
    fn should_validate_name_characters() {
        
        // Valid name characters (within 0x21 to 0x7E)
        let valid_bytes: &[u8] = &[0x21, 0x30, 0x41, 0x5A, 0x61, 0x7E];
        for &byte in valid_bytes {
            assert!(is_valid_name_char(&byte), "Byte {:X} should be valid", byte);
        }

        // Invalid name characters (outside 0x21 to 0x7E)
        let invalid_bytes: &[u8] = &[0x00, 0x20, 0x7F, 0x80, 0xFF];
        for &byte in invalid_bytes {
            assert!(!is_valid_name_char(&byte), "Byte {:X} should be invalid", byte);
        }
    }

    #[test]
    fn should_validate_name_bytes() {
        
        // Valid name bytes
        let valid_names: &[&[u8]] = &[b"/ValidName", b"/Another_Valid-Name123", b"/NameWithSymbols!@#"];
        for &name in valid_names {
            assert!(super::is_valid_name_bytes(name), "Name {:?} should be valid", name);
        }

        // Invalid name bytes
        let invalid_names: &[&[u8]] = &[b"", b"InvalidName", b"/Invalid Name ", b"/Invalid/Name", b"/Invalid\x7FName"];
        for &name in invalid_names {
            assert!(!super::is_valid_name_bytes(name), "Name {:?} should be invalid", name);
        }
    }
}