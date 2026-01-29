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
    /// Horizontal Tab character.
    /// 
    /// code: `0x09`
    HorizontalTab,
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