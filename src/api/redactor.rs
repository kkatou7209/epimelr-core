//! This module contains the core implementation of the redactor library.
use std::io::{BufReader, Read};
use std::sync::{Arc};
use std::fs::File;

use crate::byte_source::ByteSource;
use crate::byte_source::memory::MemoryByteSource;

/// A core implementation of this library.
/// <br>
/// 
/// To get the length of the content, use the `content_len` method:
/// 
/// ```rs
/// let length: usize = redactor.content_len();
/// println!("Content length: {}", length);
/// ```
#[derive(Debug, Clone)]
pub struct Redactor {
    /// The original byte source.
    source: Arc<dyn ByteSource>,
}

impl Redactor {

    /// Reads the content from the given file and creates a new `Redactor` instance.
    pub fn read(file: &File) -> std::io::Result<Self> {
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        
        Ok(Self {
            source: Arc::new(MemoryByteSource::new(buffer)),
        })
    }

    /// Creates a new `Redactor`.
    pub fn new() -> Self {
        Self {
            source: Arc::new(MemoryByteSource::new(Vec::new())),
        }
    }
    
    /// Returns the length of the content.
    pub fn content_len(&self) -> usize {

        self.source.len()
    }
}