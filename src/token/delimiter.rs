/// PDF Delimiter characters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delimiter {
    /// Indicates the start of a PDF string.
    /// 
    /// char: `(`
    LeftParenthesis,
    /// Indicates the end of a PDF string.
    /// 
    /// char: `)`
    RightParenthesis,
    /// Indicates the start of a PDF Hexadecimal string.
    /// 
    /// char: `<`
    LessThanSign,
    /// Indicates the end of a PDF hexadecimal string.
    /// 
    /// char: `>`
    GreaterThanSign,
    /// Indicates the start of an `Array`.
    /// 
    /// char: `[`
    LeftSquareBracket,
    /// Indicates the end of an `Array`.
    /// 
    /// char: `]`
    RightSquareBracket,
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