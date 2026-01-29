use std::fmt::Display;

/// PDF Integer object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    value: i32,
}

impl Integer {

    /// Creates a new `Integer` from the given bytes.
    pub fn new(int: impl Into<i32>) -> Self {

        let int = int.into();

        Self { value: int }
    }

    /// Returns the value of the Integer as i32.
    pub fn as_i32(&self) -> i32 {

        self.value
    }
}

impl Display for Integer {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}