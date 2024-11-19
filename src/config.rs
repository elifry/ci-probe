use crate::error::{Error, Result};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::yaml_parser::YamlConfig;
use crate::SupportedTask;

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
            Err(Error::Config(
                "Credentials not found. Please provide them via CLI or environment variables"
                    .to_string(),
            ))
        }
    }

    pub fn from_string(credentials: &str) -> Result<Self> {
        let parts: Vec<&str> = credentials.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::Config(
                "Invalid credentials format. Expected 'username:token'".to_string(),
            ));
        }

        Ok(Credentials {
            username: parts[0].to_string(),
            token: parts[1].to_string(),
        })
    }
}

pub trait VersionCompare {
    fn version_matches(&self, other: &str) -> bool;
}

impl VersionCompare for String {
    fn version_matches(&self, other: &str) -> bool {
        // Split versions into numeric components
        let parse_version = |v: &str| {
            let parts: Vec<_> = v.split('.').collect();
            // Check if all parts are valid numbers
            if parts.iter().all(|n| n.parse::<u32>().is_ok()) {
                Some(
                    parts
                        .iter()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        };

        match (parse_version(self), parse_version(other)) {
            (Some(v1), Some(v2)) => {
                // Both are valid version numbers, pad and compare
                let max_len = v1.len().max(v2.len());
                let v1_padded: Vec<u32> = v1
                    .into_iter()
                    .chain(std::iter::repeat(0))
                    .take(max_len)
                    .collect();
                let v2_padded: Vec<u32> = v2
                    .into_iter()
                    .chain(std::iter::repeat(0))
                    .take(max_len)
                    .collect();
                v1_padded == v2_padded
            }
            _ => {
                // One or both are invalid, fall back to string comparison
                self == other
            }
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub task_versions: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let path = path.unwrap_or_else(|| Path::new("ciprobeconfig.yml"));

        if !path.exists() {
            return Err(Error::Config(format!(
                "Config file not found at {:?}",
                path
            )));
        }

        let yaml_config = YamlConfig::load_from_file(path)?;
        let mut config = Config {
            task_versions: yaml_config.task_versions,
        };

        config.normalize_task_names();
        Ok(config)
    }

    pub fn get_all_tasks(&self) -> Vec<SupportedTask> {
        self.task_versions
            .keys()
            .map(|name| SupportedTask::Default(name.clone()))
            .collect()
    }

    fn normalize_task_names(&mut self) {
        let normalized_tasks: HashMap<String, Vec<String>> = self
            .task_versions
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect();

        self.task_versions = normalized_tasks;
    }

    pub fn get_valid_versions(&self, task_name: &str) -> Vec<&str> {
        self.task_versions
            .get(task_name)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
}
