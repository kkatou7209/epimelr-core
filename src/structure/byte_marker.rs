use crate::specification::structure::byte_marker::validate_byte_marker_value;

/// PDF Byte Marker representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteMarker {
    value: Vec<u8>,
    bytes: Vec<u8>,
}

impl ByteMarker {

    /// Creates a new `ByteMarker` from the given byte vector.
    pub fn new(value: Vec<u8>) -> Result<Self, String> {

        if let Err(e) = validate_byte_marker_value(&value) {
            
            return Err(format!("Invalid byte marker value: {:?}, error: {}", value, e));
        }

        let mut bytes = vec![b'%'];
        bytes.extend_from_slice(&value);
        
        Ok(Self { value, bytes })
    }

    /// Returns the value of the byte marker.
    pub fn value(&self) -> &[u8] {
        &self.value
    }

    /// Returns the byte representation of the byte marker.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::ByteMarker;

    #[test]
    fn should_create_valid_byte_marker() {
        let byte_marker = ByteMarker::new(b"\xE2\xE3\xCF\xD3".to_vec()).unwrap();
        assert_eq!(byte_marker.value(), b"\xE2\xE3\xCF\xD3");
        assert_eq!(byte_marker.as_bytes(), b"%\xE2\xE3\xCF\xD3");
    }
}