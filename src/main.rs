use ciprobe::{cli::Cli, cli_handler::handle_cli, error::Result};

fn main() -> Result<()> {
    let cli = Cli::parse()?;
    handle_cli(&cli)
}
