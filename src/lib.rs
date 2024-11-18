use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

// Re-export modules and types
pub mod cli;
pub mod cli_handler;
pub mod config;
pub mod git_manager;
pub mod gitversion;
pub mod pipeline_analyzer;
pub mod pipeline_detector;
pub mod report;
pub mod utils;

// Re-export commonly used types
pub use cli::Cli;
pub use cli_handler::handle_cli;
pub use config::{Config, Credentials, VersionCompare};
pub use git_manager::GitManager;
pub use gitversion::{GitVersionImplementation, GitVersionState};
pub use pipeline_analyzer::{CollectedTask, TaskImplementationCollector};
pub use pipeline_detector::{find_pipeline_files, PipelineDetector};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TaskValidState {
    Gitversion(GitVersionState),
    Default(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SupportedTask {
    // Gitversion,
    Default(String),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct TaskImplementation {
    repo_name: String,
    version: String,
    file_path: PathBuf,
}

// Add a custom parser for clap
impl std::str::FromStr for SupportedTask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // "gitversion" => Ok(SupportedTask::Gitversion),
            other => Ok(SupportedTask::Default(other.to_string())),
        }
    }
}

impl std::fmt::Display for SupportedTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // SupportedTask::Gitversion => write!(f, "gitversion"),
            SupportedTask::Default(name) => write!(f, "{}", name),
        }
    }
}

impl std::fmt::Display for TaskValidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskValidState::Gitversion(state) => write!(
                f,
                "setup@{}, execute@{}",
                state.setup_version, state.execute_version
            ),
            TaskValidState::Default(version) => write!(f, "@{}", version),
        }
    }
}

impl SupportedTask {
    // pub fn get_all_variants() -> Vec<Self> {
    //     vec![
    //         Self::Gitversion,
    //         // Add any known default tasks here if needed
    //     ]
    // }
}

pub fn format_task_states(_task: &SupportedTask, states: Vec<TaskValidState>) -> String {
    if states.is_empty() {
        return "None".to_string();
    }

    states
        .iter()
        .map(|state| format!("- {}", state))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn parse_task_name(name: &str) -> Result<SupportedTask> {
    // if name.eq_ignore_ascii_case("gitversion") {
    //     Ok(SupportedTask::Gitversion)
    // } else {
    //     Ok(SupportedTask::Default(name.to_string()))
    // }
    Ok(SupportedTask::Default(name.to_string()))
}

#[derive(Default)]
pub struct TaskIssues {
    pub missing_required_tasks: HashMap<String, Vec<String>>,
    pub invalid_states: HashMap<String, HashMap<String, Vec<TaskImplementation>>>,
    pub missing_states: HashMap<String, String>, // normalized_name, original_name
    pub all_implementations: HashMap<String, Vec<TaskImplementation>>,
    pub repos_analyzed: HashSet<String>,
    pub repos_skipped: HashSet<String>,
}

impl TaskIssues {
    pub fn add_missing_task(&mut self, repo_name: &str, task_name: &str) {
        self.missing_required_tasks
            .entry(repo_name.to_string())
            .or_default()
            .push(task_name.to_string());
    }

    pub fn add_invalid_state(
        &mut self,
        task_name: &str,
        repo_name: &str,
        version: String,
        file_path: PathBuf,
    ) {
        self.invalid_states
            .entry(task_name.to_string())
            .or_default()
            .entry(repo_name.to_string())
            .or_default()
            .push(TaskImplementation {
                repo_name: repo_name.to_string(),
                version,
                file_path,
            });
    }

    pub fn add_implementation(
        &mut self,
        task_name: &str,
        repo_name: &str,
        version: String,
        file_path: PathBuf,
        config: &Config,
        verbose: bool,
    ) {
        let normalized_task_name = task_name.to_lowercase();

        let short_repo = repo_name
            .split('/')
            .last()
            .unwrap_or(repo_name)
            .trim_end_matches("/_git/")
            .trim_end_matches(".git");

        if verbose {
            println!(
                "📝 Adding implementation for task '{}' (v{}) from repo '{}'",
                normalized_task_name, version, short_repo
            );
        }

        self.all_implementations
            .entry(normalized_task_name.clone())
            .or_default()
            .push(TaskImplementation {
                repo_name: repo_name.to_string(),
                version: version.clone(),
                file_path: file_path.clone(),
            });

        // Check if task exists in config and validate version
        // let valid_versions = if normalized_task_name.starts_with("gitversion/") {
        //     let mut versions = Vec::new();
        //     let gitversion_states = &config.task_states.gitversion;
        //     for state in gitversion_states {
        //         if normalized_task_name == "gitversion/setup" {
        //             versions.push(state.setup_version.clone());
        //         } else if normalized_task_name == "gitversion/execute" {
        //             versions.push(state.execute_version.clone());
        //         }
        //     }
        //     versions
        // } else {
        //     config
        //         .task_states
        //         .other_tasks
        //         .get(&normalized_task_name)
        //         .cloned()
        //         .unwrap_or_default()
        // };

        let valid_versions = config
            .task_states
            .other_tasks
            .get(&normalized_task_name)
            .cloned()
            .unwrap_or_default();

        if valid_versions.is_empty() {
            self.missing_states
                .insert(normalized_task_name.clone(), task_name.to_string());
        } else if !valid_versions.contains(&version) {
            self.add_invalid_state(&normalized_task_name, repo_name, version, file_path);
        }
    }
}

// Update TaskValidState equality comparison
impl PartialEq for TaskValidState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TaskValidState::Gitversion(a), TaskValidState::Gitversion(b)) => {
                a.setup_version.version_eq(&b.setup_version)
                    && a.execute_version.version_eq(&b.execute_version)
                    && a.spec_version.version_eq(&b.spec_version)
            }
            (TaskValidState::Default(a), TaskValidState::Default(b)) => a.version_eq(b),
            _ => false,
        }
    }
}
