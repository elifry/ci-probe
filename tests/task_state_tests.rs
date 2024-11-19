use ciprobe::VersionCompare;

#[test]
fn test_version_comparison() {
    // Test simple version equality
    let version1 = "2".to_string();
    let version2 = "2.0".to_string();
    let version3 = "2.0.0".to_string();

    // Test that all these versions are considered equal
    assert!(version1.version_matches(&version2));
    assert!(version2.version_matches(&version3));
    assert!(version1.version_matches(&version3));

    // Test different versions
    assert!(!version1.version_matches("3"));
    assert!(!version2.version_matches("2.1"));
    assert!(!version3.version_matches("2.0.1"));

    // Test invalid version handling
    let invalid = "not.a.version".to_string();
    assert!(!version1.version_matches(&invalid));
    assert!(invalid.version_matches(&invalid)); // Falls back to string comparison

    // Test mixed format versions
    assert!("1".to_string().version_matches("1.0"));
    assert!("1.0".to_string().version_matches("1.0.0"));
    assert!("2".to_string().version_matches("2.0.0"));

    // Test different version lengths
    assert!("1.0".to_string().version_matches("1"));
    assert!("1.0.0".to_string().version_matches("1"));
    assert!("1.0.0".to_string().version_matches("1.0"));
}
