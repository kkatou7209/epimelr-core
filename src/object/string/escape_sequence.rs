use std::fmt::Display;

/// PDF escape sequence representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeSequence {
    /// Line feed escape sequence (`\n`).
    LineFeed,
    /// Carriage return escape sequence (`\r`).
    CarriageReturn,
    /// Tab escape sequence (`\t`).
    Tab,
    /// Backspace escape sequence (`\b`).
    Backspace,
    /// Form feed escape sequence (`\f`).
    FormFeed,
    /// Parenthesis left escape sequence (`\(`).
    LeftParenthesis,
    /// Parenthesis right escape sequence (`\)`).
    RightParenthesis,
    /// Backslash escape sequence (`\\`).
    Backslash,
    /// Character code escape sequence (`\ddd`).
    CharacterCode(CharCode),
    /// Backslash only escape sequence (`\`).
    Empty,
    /// End of line escape sequence (`\<LF>`).
    /// 
    /// A Line Feed (`\n`) or Carriage Return (`\r`) immediately
    /// following a backslash (`\`) is treated as a just Line Feed.
    EndOfLine,
}

impl Display for EscapeSequence {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            EscapeSequence::LineFeed => write!(f, "\\n"),
            EscapeSequence::CarriageReturn => write!(f, "\\r"),
            EscapeSequence::Tab => write!(f, "\\t"),
            EscapeSequence::Backspace => write!(f, "\\b"),
            EscapeSequence::FormFeed => write!(f, "\\f"),
            EscapeSequence::LeftParenthesis => write!(f, "\\("),
            EscapeSequence::RightParenthesis => write!(f, "\\)"),
            EscapeSequence::Backslash => write!(f, "\\\\"),
            EscapeSequence::CharacterCode(code) => write!(f, "{}", code),
            EscapeSequence::Empty => write!(f, "\\"),
            EscapeSequence::EndOfLine => write!(f, "\\\n"),
        }
    }
}

/// PDF escaped character code representation.
/// 
/// A character code must be between `0` and `255`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharCode(u8);

impl CharCode {
    
    /// Creates a new `CharacterCode` from the given byte vector.
    pub fn new(code: u8) -> Self {

        Self(code)
    }

    /// Returns the character byte.
    pub fn as_byte(&self) -> u8 {
    
        self.0
    }
}

impl Display for CharCode {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "\\{:03}", self.0)
    }
}