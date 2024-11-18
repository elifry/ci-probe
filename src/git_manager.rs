use crate::config::Credentials;
use anyhow::{anyhow, Context, Result};
use std::{path::PathBuf, process::Command};

pub struct GitManager {
    repo_url: String,
    repo_dir: PathBuf,
    verbose: bool,
}

impl GitManager {
    pub fn new(credentials: Credentials, repo_url: &str, verbose: bool) -> Result<Self> {
        let repo_name = repo_url.split('/').last().unwrap_or("repo").to_string();

        let formatted_repo_url = if repo_url.contains("@") {
            let parts: Vec<&str> = repo_url.splitn(2, '@').collect();
            format!(
                "https://{}:{}@{}",
                credentials.username, credentials.token, parts[1]
            )
        } else {
            format!(
                "https://{}:{}@{}",
                credentials.username,
                credentials.token,
                repo_url.trim_start_matches("https://")
            )
        };

        let repo_dir = std::env::current_dir()
            .context("Failed to get current directory")?
            .join("temp_repos")
            .join(&repo_name);

        Ok(Self {
            repo_url: formatted_repo_url,
            repo_dir,
            verbose,
        })
    }

    pub fn get_repo_path(&self) -> &PathBuf {
        &self.repo_dir
    }

    // Used when --new is not provided
    pub fn ensure_repo_exists(&self) -> Result<()> {
        if self.repo_dir.exists() {
            self.update_repo()
        } else {
            self.clone_repo()
        }
    }

    // Used when --new is provided
    pub fn ensure_repo_exists_new(&self) -> Result<()> {
        self.clone_repo()
    }

    pub fn ensure_repo_exists_no_update(&self) -> Result<()> {
        if self.repo_dir.exists() {
            Ok(())
        } else {
            self.clone_repo()
        }
    }

    fn clone_repo(&self) -> Result<()> {
        let repo_name = self
            .repo_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        if self.verbose {
            println!("Cloning repository {}...", repo_name);
        }

        // Create the repository directory itself, not just the parent
        std::fs::create_dir_all(&self.repo_dir)?;

        // Initialize empty repo
        let output = Command::new("git")
            .args(["init"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to initialize repository"));
        }

        // Configure sparse checkout
        let output = Command::new("git")
            .args(["config", "core.sparseCheckout", "true"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to configure sparse checkout"));
        }

        // Create sparse-checkout file with pipeline patterns
        let sparse_patterns = [
            "*.yml",
            "*.yaml",
            "**/azure-pipelines.yml",
            "**/azure-pipelines.yaml",
            "**/*.pipeline.yml",
            "**/*.pipeline.yaml",
            ".github/workflows/*.yml",
            ".github/workflows/*.yaml",
            ".gitlab-ci.yml",
        ];

        let sparse_checkout_dir = self.repo_dir.join(".git").join("info");
        std::fs::create_dir_all(&sparse_checkout_dir)?;

        let sparse_checkout_file = sparse_checkout_dir.join("sparse-checkout");
        std::fs::write(&sparse_checkout_file, sparse_patterns.join("\n"))?;

        // Add remote
        let output = Command::new("git")
            .args(["remote", "add", "origin", &self.repo_url])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to add remote"));
        }

        // Try branches in order: develop, main, master
        let default_branch = match Self::try_fetch_branch(&self.repo_dir, "develop") {
            Ok(branch) => branch,
            Err(_) => match Self::try_fetch_branch(&self.repo_dir, "main") {
                Ok(branch) => branch,
                Err(_) => match Self::try_fetch_branch(&self.repo_dir, "master") {
                    Ok(branch) => branch,
                    Err(_) => {
                        return Err(anyhow::anyhow!(
                            "Failed to fetch repository: no default branch found"
                        ))
                    }
                },
            },
        };

        // Create and checkout the branch properly
        Command::new("git")
            .args([
                "checkout",
                "-b",
                &default_branch,
                &format!("origin/{}", default_branch),
            ])
            .current_dir(&self.repo_dir)
            .output()?;

        if self.verbose {
            println!(
                "✓ Successfully cloned repository {} with sparse checkout",
                repo_name
            );
        }
        Ok(())
    }

    // Helper function to try fetching a specific branch
    fn try_fetch_branch(repo_dir: &PathBuf, branch_name: &str) -> Result<String> {
        let output = Command::new("git")
            .args(["fetch", "--depth=1", "origin", branch_name])
            .current_dir(repo_dir)
            .output()?;

        if output.status.success() {
            Ok(branch_name.to_string())
        } else {
            Err(anyhow::anyhow!("Branch {} not found", branch_name))
        }
    }

    fn update_repo(&self) -> Result<()> {
        let repo_name = self
            .repo_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        if self.verbose {
            println!("Repository {} exists, updating...", repo_name);
        }

        // Get current branch name
        let branch_output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(&self.repo_dir)
            .output()?;

        let current_branch = String::from_utf8_lossy(&branch_output.stdout)
            .trim()
            .to_string();

        // If we're in detached HEAD state, try branches in order
        if current_branch == "HEAD" {
            let checkout_result = match Self::try_checkout_branch(&self.repo_dir, "develop") {
                Ok(_) => Ok(()),
                Err(_) => match Self::try_checkout_branch(&self.repo_dir, "main") {
                    Ok(_) => Ok(()),
                    Err(_) => Self::try_checkout_branch(&self.repo_dir, "master"),
                },
            };

            if let Err(e) = checkout_result {
                return Err(anyhow::anyhow!(
                    "Failed to checkout any default branch: {}",
                    e
                ));
            }
        }

        // Ensure sparse checkout is enabled
        let output = Command::new("git")
            .args(["config", "core.sparseCheckout", "true"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to configure sparse checkout"));
        }

        // Update sparse-checkout patterns if needed
        let sparse_patterns = [
            "*.yml",
            "*.yaml",
            "**/azure-pipelines.yml",
            "**/azure-pipelines.yaml",
            "**/*.pipeline.yml",
            "**/*.pipeline.yaml",
            ".github/workflows/*.yml",
            ".github/workflows/*.yaml",
            ".gitlab-ci.yml",
        ];

        let sparse_checkout_dir = self.repo_dir.join(".git").join("info");
        let sparse_checkout_file = sparse_checkout_dir.join("sparse-checkout");
        std::fs::write(&sparse_checkout_file, sparse_patterns.join("\n"))?;

        // Reset any local changes
        let reset_output = Command::new("git")
            .args(["reset", "--hard", "HEAD"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !reset_output.status.success() {
            println!("✗ Failed to reset repository {}", repo_name);
            return Err(anyhow::anyhow!("Failed to reset repository {}", repo_name));
        }

        // Pull latest changes
        let pull_output = Command::new("git")
            .args(["pull", "--force"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !pull_output.status.success() {
            let error = String::from_utf8_lossy(&pull_output.stderr);
            println!("✗ Failed to update repository {}", repo_name);
            println!("Error: {}", error);
            return Err(anyhow::anyhow!("Failed to update repository {}", repo_name));
        }

        if self.verbose {
            println!("✓ Successfully updated repository {}", repo_name);
        }
        Ok(())
    }

    // Helper function to try checking out a specific branch
    fn try_checkout_branch(repo_dir: &PathBuf, branch_name: &str) -> Result<()> {
        let output = Command::new("git")
            .args(["checkout", branch_name])
            .current_dir(repo_dir)
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(anyhow!("Branch {} not found", branch_name))
        }
    }
}
