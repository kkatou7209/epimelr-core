use std::ops::Deref;

/// PDF Delimiter characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    /// Indicates the start of a PDF string.
    /// 
    /// char: `(`
    LeftParen,
    /// Indicates the end of a PDF string.
    /// 
    /// char: `)`
    RightParen,
    /// Indicates the start of a PDF Hexadecimal string.
    /// 
    /// char: `<`
    LeftAngle,
    /// Indicates the end of a PDF hexadecimal string.
    /// 
    /// char: `>`
    RightAngle,
    /// Indicates the start of an `Array`.
    /// 
    /// char: `[`
    LeftSquare,
    /// Indicates the end of an `Array`.
    /// 
    /// char: `]`
    RightSquare,
    /// Indicates no special structure.
    /// 
    /// char: `{`
    LeftCurlyBracket,
    /// Indicates no special structure.
    /// 
    /// char: `}`
    RightCurlyBracket,
    /// Indicates the start of a Name object.
    /// 
    /// char: `/`
    Solidus,
    /// Indicates the start of a comment.
    /// 
    /// char: `%`
    PercentSign,
}

impl Delimiter {
    
    /// Returns the byte representation of the Delimiter.
    pub fn as_byte(&self) -> &u8 {
        match self {
            Delimiter::LeftParen => &b'(',
            Delimiter::RightParen => &b')',
            Delimiter::LeftAngle => &b'<',
            Delimiter::RightAngle => &b'>',
            Delimiter::LeftSquare => &b'[',
            Delimiter::RightSquare => &b']',
            Delimiter::LeftCurlyBracket => &b'{',
            Delimiter::RightCurlyBracket => &b'}',
            Delimiter::Solidus => &b'/',
            Delimiter::PercentSign => &b'%',
        }
    }
}

impl PartialEq<u8> for Delimiter {
    
    fn eq(&self, other: &u8) -> bool {
        self.as_byte() == other
    }
}

mod tests {
    use super::Delimiter;

    #[test]
    fn should_compare_with_byte() {
        assert_eq!(Delimiter::LeftParen, b'(');
        assert_eq!(Delimiter::RightAngle, b'>');
        assert_eq!(Delimiter::Solidus, b'/');
        assert_eq!(Delimiter::PercentSign, b'%');
        assert_eq!(Delimiter::RightCurlyBracket, b'}');
        assert_eq!(Delimiter::LeftSquare, b'[');
        assert_eq!(Delimiter::RightSquare, b']');
        assert_eq!(Delimiter::LeftAngle, b'<');
        assert_eq!(Delimiter::RightParen, b')');
        assert_eq!(Delimiter::LeftCurlyBracket, b'{');
    }
}