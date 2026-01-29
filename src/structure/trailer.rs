use std::fmt::Display;

use crate::object::{Dictionary, DictionaryValue, Name, Reference};

/// PDF Trailer representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trailer {
    /// The trailer dictionary.
    dictionary: Dictionary,
    /// The byte offset to the start of the cross-reference table.
    xref_offset: u64,
}

impl Trailer {
    
    /// Creates a new `Trailer`.
    pub fn new(
        dictionary: impl Into<Dictionary>,
        xref_offset: impl Into<u64>
    ) -> Result<Self, String> {

        let dictionary = dictionary.into();

        if dictionary.has(&Name::new("Size")) == false {
            return Err(format!("Trailer dictionary must contain '/Size' entry.: {}", dictionary));
        }

        let size = dictionary.get(&Name::new("Size")).unwrap();
        if !matches!(size, DictionaryValue::Integer(_)) {
            return Err(format!("'/Size' entry in trailer dictionary must be an Integer.: {}", dictionary));
        }

        let size = match size {
            DictionaryValue::Integer(int) => int.as_i32(),
            _ => unreachable!(),
        };

        if size < 0 {
            return Err(format!("'/Size' entry in trailer dictionary must be non-negative.: {}", dictionary));
        }

        if !dictionary.has(&Name::new("Root")) {
            return Err(format!("Trailer dictionary must contain '/Root' entry.: {}", dictionary));
        }

        let root = dictionary.get(&Name::new("Root")).unwrap();

        if !matches!(root, DictionaryValue::Reference(_)) {
            return Err(format!("'/Root' entry in trailer dictionary must be a Reference.: {}", dictionary));
        }
        
        Ok(Self {
            dictionary: dictionary.into(),
            xref_offset: xref_offset.into()
        })
    }

    /// Returns the trailer dictionary.
    pub fn dictionary_object(&self) -> &Dictionary {
        
        &self.dictionary
    }

    /// Returns the byte offset to the start of the cross-reference table.
    pub fn xref_offset(&self) -> u64 {
        
        self.xref_offset
    }

    /// Returns the size of the PDF file (number of entries in the cross-reference table).
    pub fn size(&self) -> u32 {
        
        let size = self.dictionary.get(&Name::new("Size")).unwrap();

        match size {
            DictionaryValue::Integer(int) => int.as_i32() as u32,
            _ => unreachable!(),
        }
    }

    /// Returns the root reference of the PDF file.
    pub fn root(&self) -> &Reference {
        
        let root = self.dictionary.get(&Name::new("Root")).unwrap();

        match root {
            DictionaryValue::Reference(r) => r,
            _ => unreachable!(),
        }
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