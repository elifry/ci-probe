use anyhow::Result;
use ciprobe::{cli::Cli, cli_handler::handle_cli};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();
    handle_cli(&cli)
}
