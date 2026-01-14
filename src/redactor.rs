//! This module contains the core implementation of the redactor library.
use std::{fs::File, sync::Arc};

use crate::binary::*;

/// A core implementation of this library.
/// <br>
/// 
/// For instanciating, use `Redactor::from(&File)`.
/// ```rs
/// use std::fs::File;
/// use redactor::Redactor;
/// 
/// let file = File::open("path/to/file").expect("Failed to open file");
/// let redactor = Redactor::from(&file);
/// ```
/// <br>
/// 
/// You can also create a `Redactor` instance from a file path string:
/// 
/// ```rs
/// use redactor::Redactor;
/// 
/// let redactor = Redactor::from("path/to/file");
/// ```
/// 
/// To get the length of the content, use the `content_len` method:
/// 
/// ```rs
/// let length: usize = redactor.content_len();
/// println!("Content length: {}", length);
/// ```
/// <br>
/// 
/// To get the content as a `String`, use the `as_string` method:
/// 
/// ```rs
/// let content: String = redactor.as_string();
/// println!("Content: {}", content);
/// ```
#[derive(Debug, Clone)]
pub(crate) struct Redactor {
    /// The original byte source.
    original: Arc<dyn ByteSource>,
}

impl Redactor {
    
    /// Returns the length of the content.
    pub fn content_len(&self) -> usize {
        self.original.len()
    }

    /// Returns the content as a `String`.
    pub fn as_string(&self) -> String {

        let original = Arc::clone(&self.original);
        
        String::from_utf8_lossy(&original[..]).to_string()
    }
}

impl From<&File> for Redactor {

    fn from(file: &File) -> Self {
        let source = create_source_from_file(file)
            .expect("Failed to create byte source from file");

        Self {
            original: Arc::new(source),
        }
    }
}

impl From<&str> for Redactor {
    
    fn from(value: &str) -> Self {
        let file = File::open(value).expect("Failed to open file");

        let source = create_source_from_file(&file)
            .expect("Failed to create byte source from file");

        Self {
            original: Arc::new(source),
        }
    }
}