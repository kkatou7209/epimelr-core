use crate::object::{Array, ArrayElement, Name};

/// Representation of a `/Filter` entry in a PDF stream dictionary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Filter {
    /// PDF filter name key.
    name: Name,
    /// PDF filter value.
    value: FilterValue,
}

impl Filter {

    /// Creates a new `Filter`.
    pub fn new(value: FilterValue) -> Self {

        Self {
            name: Name::new("Filter").unwrap(),
            value
        }
    }

    /// Returns the `Name` object of the Filter.
    pub fn name_object(&self) -> &Name {

        &self.name
    }

    /// Returns the `FilterValue` of the Filter.
    pub fn value_object(&self) -> &FilterValue {

        &self.value
    }

    /// Checks if the Filter is a single Name.
    pub fn is_name(&self) -> bool {

        matches!(self.value, FilterValue::Name(_))
    }

    /// Checks if the Filter is a single Name.
    pub fn is_array(&self) -> bool {

        matches!(self.value, FilterValue::Array(_))
    }
}

/// PDF filter value representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterValue {
    /// PDF single Filter Name.
    Name(FilterName),
    /// PDF array of Filter Names.
    Array(Vec<FilterName>),
}

/// PDF filter name representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterName {
    /// PDF `ASCIIHexDecode` filter.
    ASCIIHexDecode,
    /// PDF `ASCII85Decode` filter.
    ASCII85Decode,
    /// PDF `LZWDecode` filter.
    LZWDecode,
    /// PDF `FlateDecode` filter.
    FlateDecode,
    /// PDF `RunLengthDecode` filter.
    RunLengthDecode,
    /// PDF `CCITTFaxDecode` filter.
    CCITTFaxDecode,
    /// PDF `JBIG2Decode` filter.
    JBIG2Decode,
    /// PDF `DCTDecode` filter.
    DCTDecode,
    /// PDF `JPXDecode` filter.
    JPXDecode,
    /// PDF `Crypt` filter.
    Crypt,
}

impl FilterName {
    
    /// Returns the Name object of the Filter Name.
    pub fn name_object(&self) -> Name {

        match self {
            FilterName::ASCIIHexDecode => Name::new("ASCIIHexDecode").unwrap(),
            FilterName::ASCII85Decode => Name::new("ASCII85Decode").unwrap(),
            FilterName::LZWDecode => Name::new("LZWDecode").unwrap(),
            FilterName::FlateDecode => Name::new("FlateDecode").unwrap(),
            FilterName::RunLengthDecode => Name::new("RunLengthDecode").unwrap(),
            FilterName::CCITTFaxDecode => Name::new("CCITTFaxDecode").unwrap(),
            FilterName::JBIG2Decode => Name::new("JBIG2Decode").unwrap(),
            FilterName::DCTDecode => Name::new("DCTDecode").unwrap(),
            FilterName::JPXDecode => Name::new("JPXDecode").unwrap(),
            FilterName::Crypt => Name::new("Crypt").unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::object::Name;
    use crate::object::stream::filter::{Filter, FilterName, FilterValue};

    #[test]
    fn should_filter_returns_valid_name_object() {

        let filter = Filter::new(FilterValue::Name(FilterName::FlateDecode));
        
        assert_eq!(filter.name_object(), &Name::new("Filter").unwrap());
    }

    #[test]
    fn should_filter_checks_value_type() {

        let filter = Filter::new(FilterValue::Name(FilterName::FlateDecode));
        
        assert_eq!(filter.is_name(), true);
        assert_eq!(filter.is_array(), false);

        let filter = Filter::new(FilterValue::Array(vec![
            FilterName::ASCII85Decode,
            FilterName::FlateDecode,
        ]));

        assert_eq!(filter.is_name(), false);
        assert_eq!(filter.is_array(), true);
    }
}