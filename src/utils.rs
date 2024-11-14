use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::path::{Component, Path, PathBuf};

/// Sanitizes a file path to prevent directory traversal and ensure safe file operations
pub fn sanitize_file_path(path: &Path) -> PathBuf {
    let mut sanitized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(name) => sanitized.push(name),
            _ => {
                // Skip components like RootDir, ParentDir, etc.
            }
        }
    }
    sanitized
}

pub async fn load_azure_credentials() -> Result<(String, String)> {
    let (username, token) = async {
        let username_result = env::var("AZURE_USERNAME");
        let token_result = env::var("AZURE_TOKEN");

        match (username_result, token_result) {
            (Ok(username), Ok(token)) => Ok((username, token)),
            _ => Err(anyhow::anyhow!("Credentials not found in environment")),
        }
    }
    .await
    .unwrap_or_else(|_| {
        dotenv().ok();
        (
            env::var("AZURE_USERNAME").expect("AZURE_USERNAME must be set"),
            env::var("AZURE_TOKEN").expect("AZURE_TOKEN must be set"),
        )
    });

    Ok((username, token))
}
