//! This module contains structures representing various PDF structural elements.

mod comment;
mod header;
mod version;
mod byte_marker;
mod eof;

pub use crate::structure::comment::StructuralComment;
pub use crate::structure::header::Header;
pub use crate::structure::version::Version;
pub use crate::structure::byte_marker::ByteMarker;
pub use crate::structure::eof::EOF;