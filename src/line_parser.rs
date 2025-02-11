#[derive(Debug, PartialEq)]
pub struct TaskDefinition {
    pub name: String,
    pub version: String,
}

pub fn parse_task_definition(line: &str) -> Option<TaskDefinition> {
    // Trim whitespace and skip if empty
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Skip comments
    if line.starts_with('#') || line.starts_with("//") {
        return None;
    }

    // Find "task:" anywhere in the line and get everything after "task:"
    let task_pos = line.find("task:")?;
    let after_task = line[task_pos + 5..].trim();

    // Find the @ symbol
    let at_pos = after_task.find('@')?;

    // Split into name and version
    let name = after_task[..at_pos].trim();
    let version = after_task[at_pos + 1..].trim();

    // Validate name (word chars and forward slashes only)
    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '/')
        || name.is_empty()
    {
        return None;
    }

    // Validate version (digits only)
    if !version.chars().all(|c| c.is_ascii_digit()) || version.is_empty() {
        return None;
    }

    Some(TaskDefinition {
        name: name.to_string(),
        version: version.to_string(),
    })
}
