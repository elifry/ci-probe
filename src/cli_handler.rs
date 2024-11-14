use crate::{
    cli::Cli,
    config::{Config, Credentials},
    pipeline_analyzer::analyze_pipelines,
    report::generate_markdown_report,
};
use anyhow::Result;
use tokio::fs;

pub async fn handle_cli(cli: &Cli) -> Result<()> {
    // Load credentials
    let credentials = Credentials::load(&cli.credentials)?;

    if cli.verbose {
        println!("Loading configuration...");
    }

    // Load config
    let config = Config::load(cli.config_path.as_deref())?;

    // Parse repository list
    let repos: Vec<String> = cli
        .repos
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect();

    if repos.is_empty() {
        return Err(anyhow::anyhow!("No repositories specified"));
    }

    if cli.verbose {
        println!("Analyzing {} repositories...", repos.len());
    }

    // Run analysis
    let issues = analyze_pipelines(&repos, &credentials, &config, cli.verbose).await?;

    // Generate and write report
    let report = generate_markdown_report(&repos, &config, &issues).await?;

    // Write report to default location
    let output_path = "report.md";
    if cli.verbose {
        println!("Writing report to {}", output_path);
    }

    fs::write(output_path, report).await?;

    Ok(())
}
