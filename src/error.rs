use std::fmt;

#[derive(Debug)]
pub enum Error {
    Cli(String),
    Config(String),
    Git(String),
    Io(std::io::Error),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Cli(msg) => write!(f, "CLI error: {}", msg),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Git(msg) => write!(f, "Git error: {}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
