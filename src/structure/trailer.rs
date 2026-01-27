use std::fmt::Display;

use crate::object::{Dicionary, Name};

/// PDF Trailer representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trailer {
    /// The trailer dictionary.
    dictionary: Dicionary,
    /// The byte offset to the start of the cross-reference table.
    xref_offset: u64,
}

impl Trailer {
    
    /// Creates a new `Trailer`.
    pub fn new(
        dictionary: impl Into<Dicionary>,
        xref_offset: impl Into<u64>
    ) -> Result<Self, String> {

        let dictionary = dictionary.into();

        if dictionary.has(&Name::new("Size")?) == false {
            return Err(format!("Trailer dictionary must contain 'Size' entry."));
        }
        
        Ok(Self {
            dictionary: dictionary.into(),
            xref_offset: xref_offset.into()
        })
    }

    /// Returns the trailer dictionary.
    pub fn dictionary_object(&self) -> &Dicionary {
        
        &self.dictionary
    }

    /// Returns the byte offset to the start of the cross-reference table.
    pub fn xref_offset(&self) -> u64 {
        
        self.xref_offset
    }
}

impl Display for Trailer {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "trailer\n")?;
        write!(f, "{}\n", self.dictionary)?;
        write!(f, "startxref\n")?;
        write!(f, "{}\n", self.xref_offset)
    }
}