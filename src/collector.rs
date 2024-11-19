use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;

use crate::find_pipeline_files;

#[derive(Debug)]
pub enum CollectedTask {
    Regular {
        task_name: String,
        version: String,
        file_path: PathBuf,
    },
}

pub struct TaskImplementationCollector {
    pub repo_path: PathBuf,
    pub repo_name: String,
}

impl TaskImplementationCollector {
    pub fn new(repo_path: PathBuf, repo_name: String) -> Self {
        Self {
            repo_path,
            repo_name,
        }
    }

    pub fn collect(&self) -> Result<Vec<CollectedTask>> {
        let mut collected = Vec::new();
        let pipeline_files = find_pipeline_files(&self.repo_path, false)?;

        for pipeline_file in pipeline_files {
            let content = std::fs::read_to_string(&pipeline_file)?;
            let task_regex = Regex::new(r#"task:\s*([\w/]+)@(\d+)"#)?;

            let lines: Vec<&str> = content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.starts_with('#') && !line.starts_with("//"))
                .collect();

            for line in lines {
                if let Some(cap) = task_regex.captures(line) {
                    let task_name = cap[1].to_string();
                    collected.push(CollectedTask::Regular {
                        task_name,
                        version: cap[2].to_string(),
                        file_path: pipeline_file.clone(),
                    });
                }
            }
        }

        Ok(collected)
    }
}
