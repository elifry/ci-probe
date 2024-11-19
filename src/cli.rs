use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Cli {
    pub repos: String,
    pub credentials: Option<String>,
    pub config_path: Option<PathBuf>,
    pub verbose: bool,
}

impl Cli {
    pub fn parse() -> anyhow::Result<Self> {
        let mut cli = Cli::default();
        let mut args = std::env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--repos" => {
                    cli.repos = args
                        .next()
                        .ok_or_else(|| anyhow::anyhow!("--repos requires a value"))?;
                }
                "--credentials" => {
                    cli.credentials = args.next();
                }
                "--config" => {
                    cli.config_path = args.next().map(PathBuf::from);
                }
                "-v" | "--verbose" => {
                    cli.verbose = true;
                }
                "-h" | "--help" => {
                    println!("Usage: ciprobe [OPTIONS]");
                    println!("\nOptions:");
                    println!("  --repos <URLS>         Comma-separated list of repository URLs to analyze");
                    println!("  --credentials <CREDS>  Git credentials in username:token format");
                    println!("  --config <PATH>        Path to config file (default: ./ciprobeconfig.yml)");
                    println!("  -v, --verbose          Show detailed output");
                    println!("  -h, --help             Show this help message");
                    std::process::exit(0);
                }
                "-V" | "--version" => {
                    println!("ciprobe {}", env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
                _ => {
                    return Err(anyhow::anyhow!("Unknown argument: {}", arg));
                }
            }
        }

        if cli.repos.is_empty() {
            return Err(anyhow::anyhow!("--repos argument is required"));
        }

        Ok(cli)
    }
}
