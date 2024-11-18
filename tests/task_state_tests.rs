use ciprobe::TaskValidState;

mod utils_tests;

#[test]
fn test_task_valid_state_serialization() {
    let state = TaskValidState::Default("2".to_string());
    let json = serde_json::to_string(&state).unwrap();
    println!("Serialized Default: {}", json);
    let deserialized: TaskValidState = serde_json::from_str(&json).unwrap();
    assert_eq!(state, deserialized);
}
