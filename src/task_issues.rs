use crate::{Config, TaskImplementation};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

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

        let valid_versions = config
            .task_versions
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
