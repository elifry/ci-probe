use ciprobe::VersionCompare;

#[test]
fn test_version_comparison() {
    let version1 = "2".to_string();
    let version2 = "2.0".to_string();
    let version3 = "2.0.0".to_string();

    // Test that all these versions are considered equal
    assert!(version1.version_eq(&version2));
    assert!(version2.version_eq(&version3));
    assert!(version1.version_eq(&version3));

    // Test different versions
    assert!(!version1.version_eq("3"));
    assert!(!version2.version_eq("2.1"));
    assert!(!version3.version_eq("2.0.1"));

    // Test invalid version handling
    let invalid = "not.a.version".to_string();
    assert!(!version1.version_eq(&invalid));
    assert!(invalid.version_eq(&invalid)); // Falls back to string comparison
}
