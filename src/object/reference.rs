use std::fmt::Display;

/// PDF Indirect Reference representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// PDF Indirect Reference object number.
    number: u32,
    /// PDF Indirect Reference object generation.
    generation: u32,
}

impl Reference {
    
    /// Creates a new `IndirectReference` from the given number and generation.
    pub fn new(number: u32, generation: u32) -> Result<Self, String> {

        if number == 0 {
            return Err(format!("Number must be greater than 0"));
        }
        
        Ok(Self {
            number,
            generation,
        })
    }

    /// Returns the number of the Indirect Reference.
    pub fn number(&self) -> u32 {
        
        self.number
    }

    /// Returns the generation of the Indirect Reference.
    pub fn generation(&self) -> u32 {
        
        self.generation
    }
}

impl Display for Reference {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} R", self.number, self.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::Reference;

    #[test]
    fn should_equal_when_number_and_generation_are_equal() {
        
        let ref1 = Reference::new(1, 0);
        let ref2 = Reference::new(1, 0);
        
        assert_eq!(ref1, ref2);
    }   

    #[test]
    fn should_not_equal_when_number_or_generation_are_different() {
        
        let ref1 = Reference::new(1, 0);
        let ref2 = Reference::new(2, 0);
        let ref3 = Reference::new(1, 1);

        assert_ne!(ref1, ref2);
        assert_ne!(ref1, ref3);
    }
}