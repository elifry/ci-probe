use anyhow::Result;
use std::collections::HashSet;

use crate::{Config, SupportedTask, TaskIssues};

pub async fn generate_markdown_report(
    repos: &[String],
    config: &Config,
    issues: &TaskIssues,
) -> Result<String> {
    let mut md = String::new();

    generate_header(&mut md, repos, issues);
    generate_summary_section(&mut md, issues, config)?;
    if !issues.all_implementations.is_empty() {
        generate_valid_states_section(&mut md, issues, config)?;
    }
    if !issues.invalid_states.is_empty() {
        generate_invalid_states_section(&mut md, issues, config)?;
    }
    if !issues.missing_states.is_empty() {
        generate_missing_states_section(&mut md, issues)?;
    }

    Ok(md)
}

fn generate_header(md: &mut String, repos: &[String], issues: &TaskIssues) {
    let now = chrono::Local::now();
    md.push_str("# 📊 Pipeline Task Analysis Report\n\n");
    md.push_str(&format!(
        "🕒 Generated on: {}\n\n",
        now.format("%Y-%m-%d %H:%M:%S")
    ));

    // Analyzed repositories
    md.push_str("## 📚 Analyzed Repositories\n\n");
    for repo in repos {
        if !issues.repos_skipped.contains(repo) {
            let short_name = repo
                .split('/')
                .last()
                .unwrap_or(repo)
                .trim_end_matches("/_git/")
                .trim_end_matches(".git");
            md.push_str(&format!("- [{}]({})\n", short_name, repo));
        }
    }
    md.push('\n');

    // Skipped repositories
    if !issues.repos_skipped.is_empty() {
        md.push_str("## ⏭️ Skipped Repositories\n\n");
        for repo in &issues.repos_skipped {
            let short_name = repo
                .split('/')
                .last()
                .unwrap_or(repo)
                .trim_end_matches("/_git/")
                .trim_end_matches(".git");
            md.push_str(&format!(
                "- [{}]({}) (no pipeline files found)\n",
                short_name, repo
            ));
        }
        md.push('\n');
    }
}

fn generate_summary_section(md: &mut String, issues: &TaskIssues, _config: &Config) -> Result<()> {
    md.push_str("## 📈 Summary\n\n");

    // Collect unique repos that have any kind of issue
    let repos_with_issues: HashSet<_> = issues
        .missing_required_tasks
        .keys()
        .chain(issues.invalid_states.values().flat_map(|m| m.keys()))
        .chain(
            issues
                .all_implementations
                .values()
                .flat_map(|impls| impls.iter().map(|i| &i.repo_name))
                .filter(|repo| {
                    issues.missing_states.keys().any(|task| {
                        issues
                            .all_implementations
                            .get(task)
                            .map(|impls| impls.iter().any(|i| &i.repo_name == *repo))
                            .unwrap_or(false)
                    })
                }),
        )
        .filter(|repo| {
            issues.repos_analyzed.contains(*repo) && !issues.repos_skipped.contains(*repo)
        })
        .collect();

    md.push_str(&format!(
        "- 🏢 Total repositories analyzed: {}\n",
        issues.repos_analyzed.len()
    ));
    if !issues.repos_skipped.is_empty() {
        md.push_str(&format!(
            "- ⏭️ Skipped repositories: {}\n",
            issues.repos_skipped.len()
        ));
    }
    md.push_str(&format!(
        "- ⚠️ Repositories with issues: {}\n",
        repos_with_issues.len()
    ));
    md.push_str(&format!(
        "- ❌ Total missing task implementations: {}\n",
        issues
            .missing_required_tasks
            .values()
            .map(|v| v.len())
            .sum::<usize>()
    ));
    md.push_str(&format!(
        "- ⚡ Total invalid state implementations: {}\n",
        issues
            .invalid_states
            .values()
            .flat_map(|m| m.values())
            .map(|v| v.len())
            .sum::<usize>()
    ));

    md.push('\n');
    Ok(())
}

fn generate_invalid_states_section(
    md: &mut String,
    issues: &TaskIssues,
    config: &Config,
) -> Result<()> {
    md.push_str("## ⚠️ Invalid Task States\n\n");

    if issues.invalid_states.is_empty() {
        md.push_str("No invalid task states found.\n\n");
        return Ok(());
    }

    for (task_normalized, repos) in &issues.invalid_states {
        // Get original task name if it exists, otherwise use normalized
        let original_task = issues
            .missing_states
            .iter()
            .find(|(k, _)| *k == task_normalized)
            .map(|(_, v)| v.as_str())
            .unwrap_or(task_normalized);

        let valid_versions: Vec<_> = config.task_states.get_valid_versions(task_normalized);

        md.push_str(&format!(
            "### 🔧 {} (expected: {})\n\n",
            original_task,
            valid_versions.join(" or ")
        ));

        for (repo_url, implementations) in repos {
            let short_name = repo_url
                .split('/')
                .last()
                .unwrap_or(repo_url)
                .trim_end_matches("/_git/")
                .trim_end_matches(".git");
            md.push_str(&format!("#### 📁 [{}]({})\n\n", short_name, repo_url));

            for impl_ in implementations {
                let path = impl_
                    .file_path
                    .strip_prefix(std::path::Path::new("C:\\repos\\ciprobe\\temp_repos"))
                    .unwrap_or(&impl_.file_path);

                let path = if let Some(repo_name) = repo_url
                    .split('/')
                    .last()
                    .map(|s| s.trim_end_matches("/_git/").trim_end_matches(".git"))
                {
                    path.strip_prefix(repo_name).unwrap_or(path)
                } else {
                    path
                };

                md.push_str(&format!(
                    "- Version {} in `{}`\n",
                    impl_.version,
                    path.display()
                ));
            }
            md.push('\n');
        }
    }

    Ok(())
}

fn generate_missing_states_section(md: &mut String, issues: &TaskIssues) -> Result<()> {
    md.push_str("## ❌ Tasks with Missing Valid States\n\n");

    // Get original non-normalized names and sort them
    let mut original_names: Vec<_> = issues.missing_states.values().collect();
    original_names.sort();

    for task in original_names {
        md.push_str(&format!("- {}\n", task));
    }
    md.push_str("\nConsider adding these tasks to your `ciprobeconfig.yml` with the appropriate valid versions.\n\n");

    Ok(())
}

fn generate_valid_states_section(
    md: &mut String,
    issues: &TaskIssues,
    config: &Config,
) -> Result<()> {
    md.push_str("## ✅ Valid Task States\n\n");

    let mut valid_tasks = Vec::new();

    for task in config.get_all_tasks() {
        let SupportedTask::Default(task_name) = &task;

        if let Some(implementations) = issues.all_implementations.get(task_name) {
            // Only include if all implementations are valid (not in invalid_states)
            if !issues.invalid_states.contains_key(task_name) && !implementations.is_empty() {
                // Collect unique repo names
                let repos: Vec<_> = implementations
                    .iter()
                    .map(|impl_| {
                        impl_
                            .repo_name
                            .split('/')
                            .last()
                            .unwrap_or(&impl_.repo_name)
                            .trim_end_matches("/_git/")
                            .trim_end_matches(".git")
                    })
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect();

                // Get the version from the first implementation
                if let Some(first_impl) = implementations.first() {
                    valid_tasks.push(format!(
                        "- {} v{} ({})",
                        task_name,
                        first_impl.version,
                        repos.join(", ")
                    ));
                }
            }
        }
    }

    if valid_tasks.is_empty() {
        md.push_str("No tasks are currently in a valid state.\n\n");
    } else {
        valid_tasks.sort();
        for task in valid_tasks {
            md.push_str(&format!("{}\n", task));
        }
        md.push('\n');
    }

    Ok(())
}
