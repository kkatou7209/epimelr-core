use std::fmt::Display;

use crate::specification::structure::byte_marker::validate_byte_marker_value;

/// PDF Byte Marker representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteMarker {
    value: Vec<u8>,
}

impl ByteMarker {

    /// Creates a new `ByteMarker` from the given byte vector.
    pub fn new(value: Vec<u8>) -> Result<Self, String> {

        if let Err(e) = validate_byte_marker_value(&value) {
            
            return Err(format!("Invalid byte marker value: {:?}, error: {}", value, e));
        }

        Ok(Self { value})
    }

    /// Returns the value of the byte marker.
    pub fn value(&self) -> &[u8] {
        &self.value
    }

}

impl Default for ByteMarker {
    
    fn default() -> Self {
        
        Self {
            value: b"\xE2\xE3\xCF\xD3".to_vec(),
        }
    }
}

impl Display for ByteMarker {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "%")?;
        
        for byte in &self.value {
            write!(f, "{}", *byte as char)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ByteMarker;

    #[test]
    fn should_create_valid_byte_marker() {
        let byte_marker = ByteMarker::new(b"\xE2\xE3\xCF\xD3".to_vec()).unwrap();
        assert_eq!(byte_marker.value(), b"\xE2\xE3\xCF\xD3");
    }
}