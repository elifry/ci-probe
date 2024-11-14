use anyhow::Result;
use ciprobe::utils::load_azure_credentials;
use std::{env, fs, path::Path, path::PathBuf};
use tempfile::tempdir;

#[allow(dead_code)]
pub async fn copy_test_files(temp_dir: &Path, test_name: &str) -> Result<()> {
    // Get the project root directory using CARGO_MANIFEST_DIR
    let project_root = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap());

    // Check for .env file
    let env_path = project_root.join(".env");
    println!("Looking for .env at: {:?}", env_path);
    if env_path.exists() {
        println!(".env file found");
        let target_env = temp_dir.join(".env");
        println!("Copying .env to: {:?}", target_env);
        fs::copy(&env_path, &target_env)?;
    } else {
        println!(".env file not found!");
    }

    // Check for config file
    let config_name = format!("{}-ciprobeconfig.yml", test_name);
    let config_path = project_root.join("tests").join(&config_name);
    println!("Looking for config at: {:?}", config_path);
    if config_path.exists() {
        println!("Config file found");
        let target_config = temp_dir.join("ciprobeconfig.yml");
        println!("Copying config to: {:?}", target_config);
        fs::copy(&config_path, &target_config)?;
    } else {
        println!("Config file not found!");
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn setup_test_env(test_name: &str, skip_config: bool) -> Result<tempfile::TempDir> {
    println!("\n=== Setting up test environment ===");
    println!("Test name: {}", test_name);
    println!("Skip config: {}", skip_config);

    let temp_dir = tempdir()?;

    if !skip_config {
        println!("Copying test files...");
        copy_test_files(temp_dir.path(), test_name).await?;
    }

    println!("Changing to temp directory");
    env::set_current_dir(temp_dir.path())?;

    // Verify .env file is readable after directory change
    if let Ok(env_contents) = std::fs::read_to_string(".env") {
        println!(".env file contents after dir change: {}", env_contents);
    } else {
        println!("Failed to read .env file after dir change!");
    }

    if !skip_config {
        println!("Loading Azure credentials...");
        if let Ok((username, token)) = load_azure_credentials().await {
            println!("Credentials loaded for user: {}", username);
            println!("Token length: {}", token.len());
        }
    }

    println!("=== Test environment setup complete ===\n");
    Ok(temp_dir)
}
