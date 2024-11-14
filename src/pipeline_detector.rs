use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct PipelineDetector {
    patterns: Vec<String>,
}

impl PipelineDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn matches(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        path.extension()
            .map_or(false, |ext| ext == "yml" || ext == "yaml")
            && self
                .patterns
                .iter()
                .any(|pattern| path_str.contains("pipeline") || path_str.contains(pattern))
    }
}

impl Default for PipelineDetector {
    fn default() -> Self {
        Self {
            patterns: vec![
                "**/azure-pipelines.yml".to_string(),
                "**/azure-pipelines.yaml".to_string(),
                "**/*.pipeline.yml".to_string(),
                "**/*.pipeline.yaml".to_string(),
                ".github/workflows/*.yml".to_string(),
                ".github/workflows/*.yaml".to_string(),
                ".gitlab-ci.yml".to_string(),
            ],
        }
    }
}

pub async fn find_pipeline_files(repo_path: &PathBuf, verbose: bool) -> Result<Vec<PathBuf>> {
    if verbose {
        println!("Searching for pipeline files in {:?}", repo_path);
    }

    let detector = PipelineDetector::new();

    let mut pipeline_files = Vec::new();
    for entry in WalkDir::new(repo_path)
        .follow_links(true)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path().to_path_buf();
        if detector.matches(&path) {
            if verbose {
                // Strip the temp repo path and repo name
                let stripped_path = path
                    .strip_prefix(std::path::Path::new("C:\\repos\\ciprobe\\temp_repos"))
                    .unwrap_or(&path);

                let repo_name = repo_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                let final_path = stripped_path
                    .strip_prefix(repo_name)
                    .unwrap_or(stripped_path);

                println!("   └─ Found: {}", final_path.display());
            }
            pipeline_files.push(path);
        }
    }

    Ok(pipeline_files)
}
