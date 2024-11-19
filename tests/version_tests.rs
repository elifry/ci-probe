use ciprobe::VersionCompare;

#[test]
fn test_version_comparison() {
    let test_cases = vec![
        // Simple versions
        ("1", "1", true),
        ("2", "2", true),
        ("1", "2", false),
        // Dot versions
        ("1.0", "1.0", true),
        ("1.1", "1.0", false),
        ("2.0", "2.0", true),
        // Full versions
        ("1.0.0", "1.0.0", true),
        ("1.0.1", "1.0.0", false),
        ("2.0.0", "2.0.0", true),
        // Mixed formats
        ("1", "1.0", true),
        ("1", "1.0.0", true),
        ("1.0", "1.0.0", true),
        ("2", "2.0.0", true),
        ("2.0", "2.0.0", true),
    ];

    for (v1, v2, expected) in test_cases {
        assert_eq!(
            v1.to_string().version_matches(v2),
            expected,
            "Testing {} against {} (expected: {})",
            v1,
            v2,
            expected
        );
    }
}

#[test]
fn test_invalid_version_handling() {
    // Test invalid version formats
    assert!("invalid".to_string().version_matches("invalid")); // Direct string match
    assert!(!"invalid".to_string().version_matches("other")); // No match
    assert!(!"1.0".to_string().version_matches("invalid")); // Valid vs invalid
    assert!(!"invalid".to_string().version_matches("1.0")); // Invalid vs valid
    assert!(!"1.a.0".to_string().version_matches("1.0.0")); // Partially invalid
}
