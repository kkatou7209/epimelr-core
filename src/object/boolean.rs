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