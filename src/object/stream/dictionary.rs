use crate::object::stream::Filter;

/// PDF Stream Dictionary representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamDictionary {
    /// The `/Length` entry in the stream dictionary.
    length: u32,
    /// The `/Filter` entry in the stream dictionary.
    filter: Option<Filter>, 
}