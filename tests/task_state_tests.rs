use ciprobe::TaskValidState;

#[test]
fn test_task_valid_state_serialization() {
    let state = TaskValidState::Default("2".to_string());
    let yaml = serde_yaml::to_string(&state).unwrap();
    println!("Serialized Default: {}", yaml);
    let deserialized: TaskValidState = serde_yaml::from_str(&yaml).unwrap();
    assert_eq!(state, deserialized);
}
