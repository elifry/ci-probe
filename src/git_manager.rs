use crate::{
    error::{Error, Result},
    Credentials,
};
use std::{path::PathBuf, process::Command};

const SPARSE_PATTERNS: [&str; 2] = ["*.yml", "*.yaml"];

pub struct GitManager {
    repo_url: String,
    repo_dir: PathBuf,
    verbose: bool,
}

impl GitManager {
    pub fn new(credentials: Credentials, repo_url: &str, verbose: bool) -> Result<Self> {
        let repo_name = repo_url
            .split('/')
            .last()
            .unwrap_or("repo")
            .trim_end_matches("/_git/")
            .trim_end_matches(".git")
            .replace("%20", " ");

        let formatted_repo_url = if repo_url.contains("@") {
            let parts: Vec<&str> = repo_url.splitn(2, '@').collect();
            format!(
                "https://{}:{}@{}",
                credentials.username, credentials.token, parts[1]
            )
        } else {
            let url_part = repo_url.trim_start_matches("https://").replace(" ", "%20");
            format!(
                "https://{}:{}@{}",
                credentials.username,
                credentials.token,
                url_part
            )
        };

        let repo_dir = std::env::current_dir()
            .map_err(Error::Io)?
            .join("temp_repos")
            .join(&repo_name);

        Ok(Self {
            repo_url: formatted_repo_url,
            repo_dir,
            verbose,
        })
    }

    fn try_default_branches(repo_dir: &PathBuf) -> Result<String> {
        for branch in ["develop", "main", "master"] {
            if Self::try_fetch_branch(repo_dir, branch).is_ok() {
                return Ok(branch.to_string());
            }
        }
        Err(Error::Git("No default branch found".to_string()))
    }

    pub fn get_repo_path(&self) -> &PathBuf {
        &self.repo_dir
    }

    pub fn clone_or_update(&self) -> Result<()> {
        if self.repo_dir.exists() {
            self.update_repo()
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
            return Err(Error::Git("Failed to initialize repository".to_string()));
        }

        // Configure sparse checkout
        let output = Command::new("git")
            .args(["config", "core.sparseCheckout", "true"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(Error::Git(
                "Failed to configure sparse checkout".to_string(),
            ));
        }

        let sparse_checkout_dir = self.repo_dir.join(".git").join("info");
        std::fs::create_dir_all(&sparse_checkout_dir)?;

        let sparse_checkout_file = sparse_checkout_dir.join("sparse-checkout");
        std::fs::write(&sparse_checkout_file, SPARSE_PATTERNS.join("\n"))?;

        // Add remote
        let output = Command::new("git")
            .args(["remote", "add", "origin", &self.repo_url])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(Error::Git("Failed to add remote".to_string()));
        }

        let default_branch = Self::try_default_branches(&self.repo_dir)?;

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
            Err(Error::Git(format!("Branch {} not found", branch_name)))
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
            let default_branch = Self::try_default_branches(&self.repo_dir)?;
            Self::try_checkout_branch(&self.repo_dir, &default_branch)?;
        }

        // Ensure sparse checkout is enabled
        let output = Command::new("git")
            .args(["config", "core.sparseCheckout", "true"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !output.status.success() {
            return Err(Error::Git(
                "Failed to configure sparse checkout".to_string(),
            ));
        }

        let sparse_checkout_dir = self.repo_dir.join(".git").join("info");
        let sparse_checkout_file = sparse_checkout_dir.join("sparse-checkout");
        std::fs::write(&sparse_checkout_file, SPARSE_PATTERNS.join("\n"))?;

        // Reset any local changes
        let reset_output = Command::new("git")
            .args(["reset", "--hard", "HEAD"])
            .current_dir(&self.repo_dir)
            .output()?;

        if !reset_output.status.success() {
            println!("✗ Failed to reset repository {}", repo_name);
            return Err(Error::Git(format!(
                "Failed to reset repository {}",
                repo_name
            )));
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
            return Err(Error::Git(format!(
                "Failed to update repository {}",
                repo_name
            )));
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
            Err(Error::Git(format!("Branch {} not found", branch_name)))
        }
    }
}
