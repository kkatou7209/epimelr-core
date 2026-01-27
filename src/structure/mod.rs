//! This module contains structures representing various PDF structural elements.

mod byte_marker;
mod comment;
mod cross_reference;
mod eof;
mod header;
mod trailer;
mod version;

pub use byte_marker::ByteMarker;
pub use comment::StructuralComment;
pub use cross_reference::{CrossReference, CrossReferenceSection};
pub use eof::EOF;
pub use header::Header;
pub use version::Version;