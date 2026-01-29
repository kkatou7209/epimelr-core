use std::fmt::Display;

/// A PDF Real object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Real {
    /// The string representation of the real number.
    value: String,
}

impl Real {
    
    /// Creates a new `Real` from the given bytes.
    pub fn new(value: impl Into<f64>) -> Self {

        let value = value.into();
        
        Self { value: value.to_string(), }
    }

    /// Returns the value of the Real as f64.
    pub fn as_f64(&self) -> f64 {

        self.value.parse::<f64>().unwrap()
    }
}

impl Display for Real {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::Real;

    #[test]
    fn should_returns_valid_value() {

        let real = Real::new(3.14159);
        assert_eq!(real.as_f64(), 3.14159);

        let real = Real::new(-0.001);
        assert_eq!(real.as_f64(), -0.001);

        let real = Real::new(0.0000000001);
        assert_eq!(real.as_f64(), 0.0000000001);
    }
}