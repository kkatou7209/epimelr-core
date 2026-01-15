//! This module contains the core implementation of the redactor library.
use std::io::{BufReader, Read};
use std::sync::{Arc, Mutex};
use std::fs::File;

/// A core implementation of this library.
/// <br>
/// 
/// For instanciating, use `Redactor::try_from(&File)`:
/// 
/// ```rs
/// use std::fs::File;
/// use redactor::Redactor;
/// 
/// let file = File::open("path/to/file").expect("Failed to open file");
/// let redactor = Redactor::try_from(&file).expect("Failed to create Redactor");
/// ```
/// <br>
/// 
/// You can also create a `Redactor` instance from a file path string:
/// 
/// ```rs
/// use redactor::Redactor;
/// 
/// let redactor = Redactor::try_from("path/to/file").expect("Failed to create Redactor");
/// ```
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
    original: Arc<Mutex<Vec<u8>>>,
}

impl Redactor {
    
    /// Returns the length of the content.
    pub fn content_len(&self) -> usize {
        self.original.lock().unwrap().len()
    }
}

impl TryFrom<&File> for Redactor {

    type Error = std::io::Error;

    fn try_from(value: &File) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self {
            original: Arc::new(Mutex::new(data)),
        })
    }
}

impl TryFrom<&str> for Redactor {

    type Error = std::io::Error;
    
    fn try_from(value: &str) -> Result<Self, std::io::Error> {

        let data = std::fs::read(value)?;

        Ok(Self {
            original: Arc::new(Mutex::new(data)),
        })
    }
}