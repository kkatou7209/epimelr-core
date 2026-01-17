/// Checks if the given major and minor version numbers represent a valid PDF version.
/// 
/// Valid versions are 1.0 to 1.7 and 2.0.
pub fn is_valid_version(major: u8, minor: u8) -> bool {
    match (major, minor) {
        (1, 0..=7) => true,
        (2, 0..=0) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_valid_version;

    #[test]
    fn should_validate_versions() {
        let valid_versions = [(1, 0), (1, 4), (1, 7), (2, 0)];
        for (major, minor) in &valid_versions {
            assert!(is_valid_version(*major, *minor), "Version {}.{} should be valid", major, minor);
        }
    }

    #[test]
    fn should_validate_invalid_versions() {
        let invalid_versions = [(0, 9), (1, 8), (2, 1), (3, 0)];
        for (major, minor) in &invalid_versions {
            assert!(!is_valid_version(*major, *minor), "Version {}.{} should be invalid", major, minor);
        }
    }
}