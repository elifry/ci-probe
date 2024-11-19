use anyhow::Result;
use ciprobe::{cli::Cli, cli_handler::handle_cli};

fn main() -> Result<()> {
    let cli = Cli::parse()?;
    handle_cli(&cli)
}
