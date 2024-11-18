use anyhow::Result;
use dotenv::dotenv;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::{SupportedTask, TaskValidState};

#[derive(Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub token: String,
}

// First try to load from CLI, then from environment variables, then from .env file
impl Credentials {
    pub fn load(cli_credentials: &Option<String>) -> Result<Self> {
        if let Some(creds_str) = cli_credentials {
            Self::from_string(creds_str)
        } else if let (Ok(username), Ok(token)) =
            (env::var("AZURE_USERNAME"), env::var("AZURE_TOKEN"))
        {
            Ok(Credentials { username, token })
        } else {
            dotenv().ok();
            if let (Ok(username), Ok(token)) = (env::var("AZURE_USERNAME"), env::var("AZURE_TOKEN"))
            {
                Ok(Credentials { username, token })
            } else {
                Err(anyhow::anyhow!(
                    "Credentials not found in environment or .env file"
                ))
            }
        }
    }

    pub fn from_string(credentials: &str) -> Result<Self> {
        let parts: Vec<&str> = credentials.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!(
                "Invalid credentials format. Expected 'username:token'"
            ));
        }

        Ok(Credentials {
            username: parts[0].to_string(),
            token: parts[1].to_string(),
        })
    }
}

pub trait VersionCompare {
    fn version_eq(&self, other: &str) -> bool;
}

impl VersionCompare for String {
    fn version_eq(&self, other: &str) -> bool {
        // Normalize version strings
        let normalize = |v: &str| -> Result<Version> {
            // Handle versions like "1", "1.0", "1.0.0"
            let v = if v.chars().all(|c| c.is_ascii_digit()) {
                format!("{}.0.0", v)
            } else if v.matches('.').count() == 1 {
                format!("{}.0", v)
            } else {
                v.to_string()
            };
            Version::parse(&v).map_err(|e| anyhow::anyhow!("Invalid version: {}", e))
        };

        match (normalize(self), normalize(other)) {
            (Ok(v1), Ok(v2)) => v1 == v2,
            _ => self == other, // Fallback to string comparison if parsing fails
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub task_states: TaskStates,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskStates {
    #[serde(default)]
    pub other_tasks: HashMap<String, Vec<String>>,
}

impl TaskStates {
    pub fn get_valid_versions(&self, task_name: &str) -> Vec<&str> {
        self.other_tasks
            .get(task_name)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let path = path.unwrap_or_else(|| Path::new("ciprobeconfig.yml"));

        if !path.exists() {
            return Err(anyhow::anyhow!("Config file not found at {:?}", path));
        }

        let content = std::fs::read_to_string(path)?;
        let mut config: Config = serde_yaml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

        config.normalize_task_names();
        Ok(config)
    }

    pub fn get_valid_states(&self, task: &SupportedTask) -> Vec<TaskValidState> {
        match task {
            SupportedTask::Default(name) => self
                .task_states
                .other_tasks
                .get(name)
                .map(|versions| {
                    versions
                        .iter()
                        .cloned()
                        .map(TaskValidState::Default)
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    pub fn get_all_tasks(&self) -> Vec<SupportedTask> {
        let mut tasks = vec![];
        tasks.extend(
            self.task_states
                .other_tasks
                .keys()
                .map(|name| SupportedTask::Default(name.clone())),
        );
        tasks
    }

    pub fn is_valid_version(&self, task: &str, version: &str) -> bool {
        self.task_states
            .other_tasks
            .get(task)
            .map(|versions| versions.contains(&version.to_string()))
            .unwrap_or(false)
    }

    fn normalize_task_names(&mut self) {
        let normalized_tasks: HashMap<String, Vec<String>> = self
            .task_states
            .other_tasks
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect();

        self.task_states.other_tasks = normalized_tasks;
    }
}
