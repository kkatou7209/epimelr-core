use crate::structure::eof::EOF;
use crate::structure::header::Header;

/// PDF Structural Comment representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructuralComment {
    /// PDF End Of File comment representation.
    EOF(EOF),
    /// PDF Header comment representation.
    Header(Header),
}