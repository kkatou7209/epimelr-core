use crate::object::{Dictionary, DirectObject};

/// PDF object stream representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStream {
    dictionary: Dictionary,
    /// The indices of objects in the object stream.
    indices: Vec<ObjectStreamIndex>,
    /// The objects in the object stream.
    objects: Vec<DirectObject>,
}

impl ObjectStream {
    
    /// Creates a new `ObjectStream`.
    pub fn new(
        dictionary: impl Into<Dictionary>,
        indices: impl IntoIterator<Item = ObjectStreamIndex>,
        objects: impl IntoIterator<Item = DirectObject>
    ) -> Self {
        
        Self {
            dictionary: dictionary.into(),
            indices: indices.into_iter().collect(),
            objects: objects.into_iter().collect(),
        }
    }

    /// Returns the indices of the object stream.
    pub fn indices(&self) -> &[ObjectStreamIndex] {

        &self.indices
    }

    /// Returns the objects of the object stream.
    pub fn objects(&self) -> &[DirectObject] {

        &self.objects
    }
}

/// PDF object stream index representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStreamIndex {
    /// The number of object in the object stream.
    object_number: u32,
    /// The byte offset of the object in the object stream.
    offset: usize,
}

impl ObjectStreamIndex {
    
    /// Creates a new `ObjectStreamIndex`.
    pub fn new(object_number: impl Into<u32>, offset: impl Into<usize>) -> Self {
        
        Self {
            object_number: object_number.into(),
            offset: offset.into(),
        }
    }

    /// Returns the object number.
    pub fn object_number(&self) -> &u32 {

        &self.object_number
    }

    /// Returns the offset.
    pub fn offset(&self) -> &usize {

        &self.offset
    }
}