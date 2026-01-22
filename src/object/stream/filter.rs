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
            name: Name::new(b"/Filter").unwrap(),
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

    /// Returns the byte representation of the `Filter`.
    pub fn as_bytes(&self) -> Vec<u8> {

        match self.value {
            FilterValue::Name(ref filter_name) => {
                
                let mut bytes: Vec<u8> = Vec::new();

                bytes.extend_from_slice(b"/Filter ");
                bytes.extend_from_slice(&filter_name.as_bytes());

                bytes
            },
            FilterValue::Array(ref filter_names) => {
                
                let names: Vec<Name> = filter_names.iter()
                    .map(|fnm| fnm.name_object())
                    .collect();

                let array = Array::new(
                    names.into_iter()
                        .map(|n| ArrayElement::Name(n))
                        .collect()
                );

                let mut bytes: Vec<u8> = Vec::new();

                bytes.extend_from_slice(b"/Filter ");
                bytes.extend_from_slice(array.as_bytes());

                bytes
            }
        }
        
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
            FilterName::ASCIIHexDecode => Name::new(b"/ASCIIHexDecode").unwrap(),
            FilterName::ASCII85Decode => Name::new(b"/ASCII85Decode").unwrap(),
            FilterName::LZWDecode => Name::new(b"/LZWDecode").unwrap(),
            FilterName::FlateDecode => Name::new(b"/FlateDecode").unwrap(),
            FilterName::RunLengthDecode => Name::new(b"/RunLengthDecode").unwrap(),
            FilterName::CCITTFaxDecode => Name::new(b"/CCITTFaxDecode").unwrap(),
            FilterName::JBIG2Decode => Name::new(b"/JBIG2Decode").unwrap(),
            FilterName::DCTDecode => Name::new(b"/DCTDecode").unwrap(),
            FilterName::JPXDecode => Name::new(b"/JPXDecode").unwrap(),
            FilterName::Crypt => Name::new(b"/Crypt").unwrap(),
        }
    }

    /// Returns the byte representation of the Filter Name.
    pub fn as_bytes(&self) -> Vec<u8> {

        self.name_object().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::object::Name;
    use crate::object::stream::filter::{Filter, FilterName, FilterValue};

    #[test]
    fn should_filter_returns_valid_name_object() {

        let filter = Filter::new(FilterValue::Name(FilterName::FlateDecode));
        
        assert_eq!(filter.name_object(), &Name::new(b"/Filter").unwrap());
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

    #[test]
    fn should_filter_returns_valid_bytes() {

        let filter = Filter::new(FilterValue::Name(FilterName::FlateDecode));
        
        assert_eq!(filter.as_bytes(), b"/Filter /FlateDecode".to_vec());

        let filter = Filter::new(FilterValue::Array(vec![
            FilterName::ASCII85Decode,
            FilterName::FlateDecode,
        ]));

        assert_eq!(filter.as_bytes(), b"/Filter [/ASCII85Decode /FlateDecode]".to_vec());
    }

    #[test]
    fn should_filter_name_returns_valid_bytes() {

        assert_eq!(FilterName::ASCIIHexDecode.as_bytes(), b"/ASCIIHexDecode".to_vec());
        assert_eq!(FilterName::DCTDecode.as_bytes(), b"/DCTDecode".to_vec());
        assert_eq!(FilterName::FlateDecode.as_bytes(), b"/FlateDecode".to_vec());
        assert_eq!(FilterName::LZWDecode.as_bytes(), b"/LZWDecode".to_vec());
        assert_eq!(FilterName::RunLengthDecode.as_bytes(), b"/RunLengthDecode".to_vec());
        assert_eq!(FilterName::CCITTFaxDecode.as_bytes(), b"/CCITTFaxDecode".to_vec());
        assert_eq!(FilterName::JBIG2Decode.as_bytes(), b"/JBIG2Decode".to_vec());
        assert_eq!(FilterName::DCTDecode.as_bytes(), b"/DCTDecode".to_vec());
        assert_eq!(FilterName::JPXDecode.as_bytes(), b"/JPXDecode".to_vec());
        assert_eq!(FilterName::Crypt.as_bytes(), b"/Crypt".to_vec());
    }
}