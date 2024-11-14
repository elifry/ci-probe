use anyhow::Result;
use ciprobe::{cli::Cli, cli_handler::handle_cli};
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    handle_cli(&cli).await
}
