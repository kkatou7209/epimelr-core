/// Validates if the given major and minor version numbers represent 
/// a valid PDF version.
pub fn validate_version(major: u8, minor: u8) -> Result<(), String> {
    
    match (major, minor) {
        (1, 0..=7) => Ok(()),
        (2, 0..=0) => Ok(()),
        _ => Err(format!("Invalid convination of version: {}.{}", major, minor)),
    }
}

#[cfg(test)]
mod tests {
    use crate::specification::structure::version::validate_version;


    #[test]
    fn should_validate_versions() {
        let valid_versions = [(1, 0), (1, 4), (1, 7), (2, 0)];
        for (major, minor) in &valid_versions {
            assert!(validate_version(*major, *minor).is_ok(), "Version {}.{} should be valid", major, minor);
        }
    }

    #[test]
    fn should_validate_invalid_versions() {
        let invalid_versions = [(0, 9), (1, 8), (2, 1), (3, 0)];
        for (major, minor) in &invalid_versions {
            assert!(validate_version(*major, *minor).is_err(), "Version {}.{} should be invalid", major, minor);
        }
    }
}