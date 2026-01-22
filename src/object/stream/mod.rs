mod dictionary;
mod filter;
mod length;

pub use dictionary::StreamDictionary;
pub use filter::{Filter, FilterName, FilterValue};
pub use length::Length;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {

}