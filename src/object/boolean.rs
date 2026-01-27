use std::fmt::Display;

/// PDF Boolean object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean(bool);

impl Boolean {

    /// Creates a new `Boolean` from the given Rust bool.
    pub fn new(value: impl Into<bool>) -> Self {
        
        Self(value.into())
    }

    /// Creates a new `Boolean` representing `true`.
    pub fn truethy() -> Self {
        Self(true)
    }

    /// Creates a new `Boolean` representing `false`.
    pub fn falsy() -> Self {
        Self(false)
    }

    /// Returns the Rust bool.
    pub fn as_bool(&self) -> bool {
        
        self.0
    }
}

impl Display for Boolean {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        if self.0 {
            return write!(f, "true")
        }

        write!(f, "false")
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;

    #[test]
    fn should_create_true_boolean() {
        let boolean = Boolean::new(true);
        assert_eq!(boolean.as_bool(), true);
    }

    #[test]
    fn should_create_false_boolean() {
        let boolean = Boolean::new(false);
        assert_eq!(boolean.as_bool(), false);
    }
}