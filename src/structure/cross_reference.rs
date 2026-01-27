use std::fmt::Display;

/// PDF cross reference table representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossReferenceTable {
    /// The sections of the cross reference table.
    sections: Vec<CrossReferenceSection>,
}

impl CrossReferenceTable {
    
    /// Creats new `CrossReferenceTable`.
    pub fn new(sections: impl IntoIterator<Item = CrossReferenceSection>) -> Self {

        let mut sections: Vec<CrossReferenceSection> = sections.into_iter().collect();

        sections.sort_by(|a, b| a.start_object_number().cmp(&b.start_object_number()));

        Self {
            sections,
        }
    }

    /// Returns the sections of the Cross Reference Table.
    pub fn sections(&self) -> &[CrossReferenceSection] {

        &self.sections
    }

    /// Returns the reference for the given object number.
    pub fn reference_of_number(&self, number: impl Into<u32>) -> Option<&CrossReference> {

        let number = number.into();

        for section in &self.sections {

            if let Some(reference) = section.reference_of_number(number) {
                return Some(reference);
            }
        }

        None        
    }
}

impl Display for CrossReferenceTable {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for section in &self.sections {
            write!(f, "{}", section)?;
        }

        Ok(())
    }
}

/// PDF cross reference section representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossReferenceSection {
    /// The starting object number of the cross reference table.
    start_object_number: u32,
    /// The cross references of the table.
    entries: Vec<CrossReference>,
}

impl CrossReferenceSection {
    
    /// Creates a new `CrossReferenceTable`.
    pub fn new(
        start_object_number: impl Into<u32>,
        entries: impl IntoIterator<Item = CrossReference>,
    ) -> Self {

        Self {
            start_object_number: start_object_number.into(),
            entries: entries.into_iter().collect(),
        }
    }

    /// Returns the starting object number of the Cross Reference Table.
    pub fn start_object_number(&self) -> u32 {

        self.start_object_number
    }

    /// Returns the length of the Cross Reference Table.
    pub fn len(&self) -> usize {

        self.entries.len()
    }

    /// Returns the Cross Reference for the given object number.
    pub fn reference_of_number(&self, number: impl Into<u32>) -> Option<&CrossReference> {

        let index = number.into() - self.start_object_number;

        if let Some(reference) = self.entries.get(index as usize) {

            Some(reference)

        } else {

            None
        }
    }
}

impl Display for CrossReferenceSection {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{} {}\n", self.start_object_number, self.entries.len())?;

        for entry in &self.entries {
            write!(f, "{}\n", entry)?;
        }

        Ok(())
    }
}

/// PDF Cross Reference representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossReference {
    /// The byte offset of the object in the PDF file.
    offset: u64,
    /// The generation number of cross reference.
    generation: u32,
    /// Whether the object is in use or free.
    in_use: bool,
}

impl CrossReference {
    
    /// Creates a new `CrossReference`.
    pub fn new(
        offset: impl Into<u64>,
        generation: impl Into<u32>,
        in_use: impl Into<bool>,
    ) -> Result<Self, String> {

        let offset = offset.into();
        let generation = generation.into();
        let in_use = in_use.into();

        if offset > 99999999999 {
            return Err(format!("Offset must be between 0 and 9999999999"));
        }

        if generation > 65535 {
            return Err(format!("Generation must be between 0 and 65535"));
        }

        Ok(Self {
            offset,
            generation,
            in_use,
        })
    }

    /// Creates a new free `CrossReference`.
    pub fn free(
        offset: u64,
        generation: u32,
    ) -> Result<Self, String> {

        Self::new(offset, generation, false)
    }

    /// Creates a new in-use `CrossReference`.
    pub fn in_use(
        offset: u64,
        generation: u32,
    ) -> Result<Self, String> {

        Self::new(offset, generation, true)
    }

    /// Returns the offset of the Cross Reference.
    pub fn offset(&self) -> u64 {

        self.offset
    }

    /// Returns the generation of the Cross Reference.
    pub fn generation(&self) -> u32 {

        self.generation
    }

    /// Returns whether the Cross Reference is in use.
    pub fn is_in_use(&self) -> bool {

        self.in_use
    }
}

impl Display for CrossReference {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let in_use_char = if self.in_use { 'n' } else { 'f' };

        write!(f, "{:0>10} {:0>5} {} ", self.offset, self.generation, in_use_char)
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::{CrossReference, CrossReferenceSection};
    use crate::structure::cross_reference::CrossReferenceTable;


    #[test]
    fn should_return_reference_of_given_object_number() {

        let table = CrossReferenceTable::new(vec![
            CrossReferenceSection::new(1_u32, vec![
                CrossReference::new(0_u64, 65535_u32, false).unwrap(),
                CrossReference::new(15_u64, 0_u32, true).unwrap(),
            ]),
            CrossReferenceSection::new(3_u32, vec![
                CrossReference::new(45_u64, 0_u32, true).unwrap(),
                CrossReference::new(78_u64, 0_u32, true).unwrap(),
            ]),
        ]);

        let reference = table.reference_of_number(2_u32);

        assert_eq!(reference, Some(&CrossReference::new(15_u64, 0_u32, true).unwrap()));

        let reference = table.reference_of_number(4_u32);

        assert_eq!(reference, Some(&CrossReference::new(78_u64, 0_u32, true).unwrap()));
    }

    #[test]
    fn should_error_if_offset_or_generation_are_out_of_bounds() {

        let reference = CrossReference::new(100000000000_u64, 0_u32, true);

        assert!(reference.is_err());

        let reference = CrossReference::new(0_u64, 70000_u32, true);
        assert!(reference.is_err());
    }
}