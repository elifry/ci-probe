use anyhow::Result;
use clap::Parser;
use ciprobe::{cli::Cli, cli_handler::handle_cli};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    handle_cli(&cli).await
}
