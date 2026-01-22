mod character_code;

use character_code::CharacterCode;

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
    CharacterCode(CharacterCode),
    /// Backslash only escape sequence (`\`).
    Empty,
    /// End of line escape sequence (`\<LF>`).
    /// 
    /// A Line Feed (`\n`) or Carriage Return (`\r`) immediately
    /// following a backslash (`\`) is treated as a just Line Feed.
    EndOfLine,
}