use crate::{
    analyzer::analyze_pipelines,
    cli::Cli,
    config::{Config, Credentials},
    report::generate_markdown_report,
};
use anyhow::Result;
use tokio::fs;

pub async fn handle_cli(cli: &Cli) -> Result<()> {
    let credentials = Credentials::load(&cli.credentials)?;

    if cli.verbose {
        println!("Loading configuration...");
    }

    let config = Config::load(cli.config_path.as_deref())?;

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

    let issues = analyze_pipelines(&repos, &credentials, &config, cli.verbose).await?;

    let report = generate_markdown_report(&repos, &config, &issues).await?;

    let output_path = "report.md";
    if cli.verbose {
        println!("Writing report to {}", output_path);
    }

    fs::write(output_path, report).await?;

    Ok(())
}
