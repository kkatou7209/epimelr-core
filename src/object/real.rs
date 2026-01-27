use std::fmt::Display;

/// A PDF Real object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Real {
    /// The string representation of the real number.
    value: String,
}

impl Real {
    
    /// Creates a new `Real` from the given bytes.
    pub fn new(value: impl Into<String>) -> Result<Self, String> {

        let value_str = value.into();

        if value_str.parse::<f64>().is_err() {
            return Err(format!("Invalid real number format: {}", value_str));
        }
        
        Ok(Self { value: value_str, })
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
    fn should_create_valid_real() {
        let real = Real::new("3.14");
        assert_eq!(real.is_ok(), true);

        let real = Real::new("-0.001");
        assert_eq!(real.is_ok(), true);

        let real = Real::new("-.0");
        assert_eq!(real.is_ok(), true);

        let real = Real::new("+1.");
        assert_eq!(real.is_ok(), true);

        let real = Real::new("invalid_real");
        assert_eq!(real.is_err(), true);

        let real = Real::new("-.");
        assert_eq!(real.is_err(), true);
    }

    #[test]
    fn should_returns_valid_value() {

        let real = Real::new("3.14159").unwrap();
        assert_eq!(real.as_f64(), 3.14159);

        let real = Real::new("-0.001").unwrap();
        assert_eq!(real.as_f64(), -0.001);

        let real = Real::new("0.0000000001").unwrap();
        assert_eq!(real.as_f64(), 0.0000000001);
    }

    #[test]
    fn should_format_real_correctly() {

        let real = Real::new("2.71828").unwrap();
        assert_eq!(format!("{}", real), "2.71828");

        let real = Real::new("-123.456").unwrap();
        assert_eq!(format!("{}", real), "-123.456");

        let real = Real::new("0.0").unwrap();
        assert_eq!(format!("{}", real), "0.0");
    }
}