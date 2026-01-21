use crate::object::{Array, Dicionary, Name};

// PDF Stream object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    /// The object number of the Stream.
    number: u32,
    /// The generation number of the Stream.
    generation: u32,
    /// The dictionary associated with the Stream.
    dictionary: StreamDictionary,
    /// The content bytes of the Stream.
    content: Vec<u8>,
}

/// PDF Stream Dictionary representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamDictionary {
    /// The length of the Stream content.
    length: u32,
    /// The optional filter applied to the Stream.
    filter: Option<Filter>,
    /// The optional decode parameters for the Stream.
    decode_parms: Option<DecodeParam>,
}

/// PDF Stream Filter representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    /// A single Name as Filter.
    Name(Name),
    /// An array of Names as Filter.
    Array(Array),
}

/// PDF Stream DecodeParms representation.
impl Filter {
    
    /// Returns the Name of the Filter entry.
    pub fn name_object() -> Name {
        Name::new(b"/Filter").unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeParam {
    /// A single Dicionary as DecodeParms.
    Dicionary(Dicionary),
    /// An array of Dicionaries as DecodeParms.
    Array(Vec<Dicionary>),
}

impl DecodeParam {
    
    /// Returns the Name of the DecodeParms entry.
    pub fn name_object() -> Name {
        Name::new(b"/DecodeParms").unwrap()
    }
}