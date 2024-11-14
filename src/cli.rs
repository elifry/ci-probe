use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Default)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Comma-separated list of repository URLs to analyze
    #[arg(long = "repos", required = true)]
    pub repos: String,

    /// Git credentials in username:token format (overrides environment variables)
    #[arg(long = "credentials")]
    pub credentials: Option<String>,

    /// Path to config file (defaults to ./ciprobeconfig.yml)
    #[arg(long = "config")]
    pub config_path: Option<PathBuf>,

    /// Show detailed output
    #[arg(short, long)]
    pub verbose: bool,
}
