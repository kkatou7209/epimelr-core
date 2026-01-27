use std::fmt::Display;

use crate::object::Dicionary;

/// PDD stream object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    /// The stream dictionary.
    dictionary: Dicionary,
    /// The stream content.
    content: Vec<u8>,
}

impl Stream {
    
    /// Creates new `Stream` object.
    pub fn new(dictionary: Dicionary, content: Vec<u8>) -> Self {
        
        Self { dictionary, content }
    }

    /// Returns `Dictionary` of stream.
    pub fn dictionary(&self) -> &Dicionary {
        
        &self.dictionary
    }

    /// Returns content of stream.
    pub fn content(&self) -> &[u8] {
        
        &self.content
    }
}

impl Display for Stream {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "{}\nstream\n", self.dictionary)?;
        
        for &byte in &self.content {
            write!(f, "{}", byte as char)?;
        }
        
        write!(f, "\nendstream")
    }
}