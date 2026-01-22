/// A PDF Real object representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Real {
    value: i128,
    padding: i64,
}

pub const REAL_MAX_DIGITS: i64 = 15;

impl Real {
    
    /// Creates a new `Real` from the given bytes.
    pub fn new(value: f64) -> Result<Self, String> {

        if value.is_infinite() || value.is_nan() {
            return Err(format!("Invalid real number: value = {:?}", value));
        }

        let str = format!("{}", value.abs());

        let digit = str.len() as i64;

        let padding = ((REAL_MAX_DIGITS - digit) * 10_i64) as f64;

        let value = (value * padding) as i128;

        Ok(Self { value, padding: padding as i64 })
    }

    /// Returns the value of the Real as f64.
    pub fn as_f64(&self) -> f64 {

        self.value as f64 / self.padding as f64
    }
}