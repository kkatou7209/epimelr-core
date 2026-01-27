use std::fmt::Display;

/// PDF Integer object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer {
    value: String,
}

impl Integer {

    /// Creates a new `Integer` from the given bytes.
    pub fn new(int: impl Into<String>) -> Result<Self, String> {

        let int_str = int.into();

        if int_str.parse::<i32>().is_err() {
            return Err(format!("Invalid integer format: {}", int_str));
        }

        Ok(Self { value: int_str })
    }

    /// Returns the value of the Integer as i32.
    pub fn as_i32(&self) -> i32 {

        self.value.parse::<i32>().unwrap()
    }
}

impl Display for Integer {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}