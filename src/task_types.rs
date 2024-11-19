use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::config::VersionCompare;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TaskValidState {
    Default(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SupportedTask {
    Default(String),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TaskImplementation {
    pub repo_name: String,
    pub version: String,
    pub file_path: PathBuf,
}

// Add a custom parser for clap
impl std::str::FromStr for SupportedTask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = s.to_lowercase();
        let other = binding.as_str();
        Ok(SupportedTask::Default(other.to_string()))
    }
}

impl std::fmt::Display for SupportedTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedTask::Default(name) => write!(f, "{}", name),
        }
    }
}

impl std::fmt::Display for TaskValidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskValidState::Default(version) => write!(f, "@{}", version),
        }
    }
}

impl PartialEq for TaskValidState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TaskValidState::Default(a), TaskValidState::Default(b)) => a.version_eq(b),
        }
    }
}
