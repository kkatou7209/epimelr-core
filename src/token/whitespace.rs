use std::ops::Deref;

/// PDF Whitespace characters representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Whitespace {
    /// Null character.
    /// 
    /// code: `0x00`
    Null,
    /// Space character.
    /// 
    /// code: `0x20`
    Space,
    /// Tab character.
    /// 
    /// code: `0x09`
    Tab,
    /// LF(Line Feed) character.
    /// 
    /// code: `0x0A`
    LineFeed,
    /// FF(Form Feed) character.
    /// 
    /// code: `0x0C`
    FormFeed,
    /// CR(Carriage Return) character.
    /// 
    /// code: `0x0D`
    CarriageReturn,
}

impl Whitespace {
    
    /// Returns the byte representation of the Whitespace character.
    pub fn as_byte(&self) -> &u8 {
        match self {
            Whitespace::Null => &0x00,
            Whitespace::Space => &0x20,
            Whitespace::Tab => &0x09,
            Whitespace::LineFeed => &0x0A,
            Whitespace::FormFeed => &0x0C,
            Whitespace::CarriageReturn => &0x0D,
        }
    }
}

impl PartialEq<u8> for Whitespace {
    
    fn eq(&self, other: &u8) -> bool {
        self.as_byte() == other
    }
}

#[cfg(test)]
mod tests {
    use super::Whitespace;

    #[test]
    fn should_compare_whitespace_with_byte() {
        assert_eq!(Whitespace::Null, 0x00);
        assert_eq!(Whitespace::Space, 0x20);
        assert_eq!(Whitespace::Tab, 0x09);
        assert_eq!(Whitespace::LineFeed, 0x0A);
        assert_eq!(Whitespace::FormFeed, 0x0C);
        assert_eq!(Whitespace::CarriageReturn, 0x0D);
    }
}