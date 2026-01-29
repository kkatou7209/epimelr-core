use crate::filter::Filter;

/// ASCIIHexDecode filter implementation.
/// 
/// This filter does not handle EOD (End of Data) marker.
/// 
/// Whitespaces and non-hexadecimal characters are ignored during decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsciiHexDecode;

impl Filter for AsciiHexDecode {
    
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>, String> {

        let mut result = Vec::with_capacity(data.len() / 2);

        let data = data.iter()
            .cloned()
            .filter(|b| b.is_ascii_hexdigit())
            .collect::<Vec<u8>>();

        for byte_pair in data.chunks(2) {
            
            let left = match byte_pair.get(0) {
                Some(&byte) => {
                    match byte {
                        b'0'..=b'9' => (byte - b'0') << 4,
                        b'A'..=b'F' => (byte - b'A' + 0xA) << 4,
                        b'a'..=b'f' => (byte - b'a' + 0xA) << 4,
                        _ => 0,
                    }
                },
                _ => unreachable!(),
            };

            let right = match byte_pair.get(1) {
                Some(&byte) => {
                    match byte {
                        b'0'..=b'9' => byte - b'0',
                        b'A'..=b'F' => byte - b'A' + 0xA,
                        b'a'..=b'f' => byte - b'a' + 0xA,
                        _ => 0,
                    }
                },
                None => 0,
                _ => unreachable!(),
            };

            result.push(left | right);
        }

        Ok(result)
    }

    fn encode(&self, data: &[u8]) -> Result<Vec<u8>, String> {

        let mut result = Vec::with_capacity(data.len() * 2);

        for byte in data {
            let left = (byte >> 4) & 0x0F;
            let right = byte & 0x0F;

            let left_char = if left < 0xA {
                b'0' + left
            } else {
                b'A' + (left - 0xA)
            };

            let right_char = if right < 0xA {
                b'0' + right
            } else {
                b'A' + (right - 0xA)
            };

            result.push(left_char);
            result.push(right_char);
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::filter::{Filter, AsciiHexDecode};

    
    #[test]
    fn should_decode_ascii_hex_encoded_data() {

        let filter = AsciiHexDecode;

        let data = b"48656C6C6F20576F726C6421";

        let decoded = filter.decode(data).unwrap();

        assert_eq!(decoded, &[0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21]);
    }

    #[test]
    fn should_encode_data_to_ascii_hex() {

        let filter = AsciiHexDecode;

        let data = &[0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21];

        let encoded = filter.encode(data).unwrap();

        assert_eq!(encoded, b"48656C6C6F20576F726C6421");
    }

    #[test]
    fn should_ignore_whitespaces_or_none_hex_characters_when_decoding() {
        let filter = AsciiHexDecode;

        let data = b"48 65 6C 6C 6F 20 57 6F 72 6C 64 21";

        let decoded = filter.decode(data).unwrap();

        assert_eq!(decoded, &[0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21]);

        let data = b"48G65H6C6I6F2J057K6F7L2M6C6N6421";

        let decoded = filter.decode(data).unwrap();

        assert_eq!(decoded, &[0x48, 0x65, 0x6C, 0x66, 0xF2, 0x05, 0x76, 0xF7, 0x26, 0xC6, 0x64, 0x21]);
    }

}