use ciprobe::line_parser::{parse_task_definition, TaskDefinition};

#[test]
fn test_parse_task_definition() {
    // Valid cases
    assert_eq!(
        parse_task_definition("task: my/task@123"),
        Some(TaskDefinition {
            name: "my/task".to_string(),
            version: "123".to_string(),
        })
    );

    assert_eq!(
        parse_task_definition("task:simple_task@1"),
        Some(TaskDefinition {
            name: "simple_task".to_string(),
            version: "1".to_string(),
        })
    );

    // Invalid cases
    assert_eq!(parse_task_definition(""), None);
    assert_eq!(parse_task_definition("task:"), None);
    assert_eq!(parse_task_definition("task: @1"), None);
    assert_eq!(parse_task_definition("task: name@"), None);
    assert_eq!(parse_task_definition("task: name@abc"), None);
    assert_eq!(parse_task_definition("other: name@123"), None);
    assert_eq!(parse_task_definition("task: invalid!name@123"), None);
}

#[test]
fn test_whitespace_handling() {
    assert_eq!(
        parse_task_definition("   task:    my_task   @   123   "),
        Some(TaskDefinition {
            name: "my_task".to_string(),
            version: "123".to_string(),
        })
    );
}
