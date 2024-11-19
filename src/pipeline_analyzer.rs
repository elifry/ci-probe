use anyhow::Result;
use regex::Regex;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::{find_pipeline_files, Config, Credentials, GitManager, SupportedTask, TaskIssues};

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
    pub async fn collect(&self) -> Result<Vec<CollectedTask>> {
        let mut collected = Vec::new();
        let pipeline_files = find_pipeline_files(&self.repo_path, false).await?;

        for pipeline_file in pipeline_files {
            let content = std::fs::read_to_string(&pipeline_file)?;
            let task_regex = Regex::new(r#"task:\s*([\w/]+)@(\d+)"#)?;

            let lines: Vec<&str> = content
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.starts_with('#') && !line.starts_with("//"))
                .collect();

            // Handle regular tasks
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

pub async fn analyze_pipelines(
    repos: &[String],
    credentials: &Credentials,
    config: &Config,
    verbose: bool,
) -> Result<TaskIssues> {
    println!("🔍 Analyzing {} repositories...", repos.len());

    let mut issues = TaskIssues::default();
    let all_tasks: HashSet<_> = config.get_all_tasks().into_iter().collect();

    for repo_url in repos {
        if verbose {
            println!("\n📂 Analyzing {}", repo_url);
        }

        let git_manager = GitManager::new(credentials.clone(), repo_url, verbose)?;

        match analyze_single_repo(
            repo_url,
            &git_manager,
            config,
            &all_tasks,
            &mut issues,
            verbose,
        )
        .await
        {
            Ok(repo_tasks) => {
                // Store implementations and check for missing tasks
                for task in &all_tasks {
                    let task_name = task.to_string();
                    if !repo_tasks.contains(&task_name) {
                        issues.add_missing_task(repo_url, &task_name);
                    }
                }
            }
            Err(e) => {
                println!("Error analyzing repository {}: {}", repo_url, e);
                continue;
            }
        }
    }

    println!("\n✅ Analysis complete");

    Ok(issues)
}

async fn analyze_single_repo(
    repo_url: &str,
    git_manager: &GitManager,
    config: &Config,
    _all_tasks: &HashSet<SupportedTask>,
    issues: &mut TaskIssues,
    verbose: bool,
) -> Result<HashSet<String>> {
    let short_name = repo_url
        .split('/')
        .last()
        .unwrap_or(repo_url)
        .trim_end_matches("/_git/")
        .trim_end_matches(".git");

    if !verbose {
        println!("\n📂 Analyzing {}", short_name);
    }

    git_manager.clone_or_update()?;
    let pipeline_files = find_pipeline_files(git_manager.get_repo_path(), verbose).await?;

    // Add to analyzed repos regardless of whether we find pipeline files
    issues.repos_analyzed.insert(repo_url.to_string());

    if pipeline_files.is_empty() {
        if !verbose {
            println!("   └─ Found 0 pipeline files, skipping");
        }
        issues.repos_skipped.insert(repo_url.to_string());
        return Ok(HashSet::new());
    }

    if !verbose {
        println!("   └─ Found {} pipeline files", pipeline_files.len());
    }

    let mut found_tasks = HashSet::new();
    for file in &pipeline_files {
        let tasks = analyze_pipeline_file(file, verbose).await?;
        for task_with_version in &tasks {
            if let Some((task_name, version)) = task_with_version.split_once('@') {
                if verbose {
                    println!(
                        "   └─ 📝 Processing implementation: {} @ {}",
                        task_name, version
                    );
                }
                issues.add_implementation(
                    task_name,
                    repo_url,
                    version.to_string(),
                    file.clone(),
                    config,
                    verbose,
                );
                found_tasks.insert(task_name.to_string());
            }
        }
    }

    if !verbose {
        println!("   └─ Found {} unique tasks", found_tasks.len());
    }

    Ok(found_tasks)
}

async fn analyze_pipeline_file(file_path: &PathBuf, verbose: bool) -> Result<HashSet<String>> {
    let content = tokio::fs::read_to_string(file_path).await?;
    let task_regex = Regex::new(r#"task:\s*([\w/]+)@(\d+)"#)?;
    let mut found_tasks = HashSet::new();

    if verbose {
        let relative_path = file_path
            .strip_prefix(std::path::Path::new("C:\\repos\\ciprobe\\temp_repos"))
            .unwrap_or(file_path);
        println!("📄 Analyzing pipeline file: {}", relative_path.display());
    }

    for line in content.lines() {
        if let Some(cap) = task_regex.captures(line.trim()) {
            let task_name = cap[1].to_string();
            let version = cap[2].to_string();
            let task_with_version = format!("{}@{}", task_name, version);

            if verbose {
                println!("   ├─ 🔍 Found task: {} @ version {}", task_name, version);
            }

            found_tasks.insert(task_with_version);
        }
    }

    if verbose && found_tasks.is_empty() {
        println!("   └─ No tasks found in this file");
    }

    Ok(found_tasks)
}
