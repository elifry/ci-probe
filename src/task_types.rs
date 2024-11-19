use std::path::PathBuf;

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
