/// PDF Boolean object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean(bool);

impl Boolean {

    /// Creates a new `Boolean` from the given Rust bool.
    pub fn new(value: bool) -> Self {
        
        Self(value)
    }

    /// Returns the Rust bool.
    pub fn as_bool(&self) -> bool {
        
        self.0
    }

    /// Returns the byte representation of the Boolean.
    pub fn as_bytes(&self) -> &[u8] {

        if self.0 {
            b"true"
        } else {
            b"false"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Boolean;

    #[test]
    fn should_create_true_boolean() {
        let boolean = Boolean::new(true);
        assert_eq!(boolean.as_bool(), true);
        assert_eq!(boolean.as_bytes(), b"true");
    }

    #[test]
    fn should_create_false_boolean() {
        let boolean = Boolean::new(false);
        assert_eq!(boolean.as_bool(), false);
        assert_eq!(boolean.as_bytes(), b"false");
    }
}