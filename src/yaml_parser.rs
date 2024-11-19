use crate::error::{Error, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct YamlConfig {
    pub task_versions: HashMap<String, Vec<String>>,
}

impl YamlConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::Config(format!("Failed to read config file: {}", e)))?;

        if !content.contains("task_versions:") {
            return Err(Error::Config(
                "Invalid config file: missing task_versions section".to_string(),
            ));
        }

        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self> {
        let mut task_versions = HashMap::new();
        let mut current_indent = 0;
        let mut in_task_versions = false;
        let mut current_task = String::new();

        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let indent = line.len() - trimmed.len();

            if trimmed.starts_with("task_versions:") {
                in_task_versions = true;
                current_indent = indent;
                continue;
            }

            if in_task_versions && indent > current_indent {
                if trimmed.ends_with(':') {
                    let task_name = trimmed
                        .trim_end_matches(':')
                        .trim_matches('\'')
                        .trim_matches('"')
                        .trim();
                    current_task = task_name.to_string();
                    task_versions.insert(current_task.clone(), Vec::new());
                } else if trimmed.starts_with('-') {
                    let version = trimmed
                        .trim_start_matches('-')
                        .trim()
                        .trim_matches('\'')
                        .trim_matches('"')
                        .trim();
                    if !version.is_empty() && !current_task.is_empty() {
                        if let Some(versions) = task_versions.get_mut(&current_task) {
                            versions.push(version.to_string());
                        }
                    }
                }
            } else if indent <= current_indent {
                in_task_versions = false;
            }
        }

        if task_versions.is_empty() {
            return Err(Error::Config(
                "No valid task versions found in config".to_string(),
            ));
        }

        Ok(YamlConfig { task_versions })
    }
}
