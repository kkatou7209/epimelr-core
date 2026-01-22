/// PDF Integer object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    value: i32,
}

impl Integer {

    /// Creates a new `Integer` from the given bytes.
    pub fn new(int: i32) -> Result<Self, String> {

        Ok(Self { value: int })
    }

    /// Returns the value of the Integer as i32.
    pub fn as_i32(&self) -> i32 {

        self.value as i32
    }
}