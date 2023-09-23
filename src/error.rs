use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FResult},
    process::exit,
};

#[derive(Debug)]
pub enum CliError {
    CommandFailed,
    PluginNotFound,
    PaymentRequired,
    Error(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::CommandFailed => write!(f, "command failed"),
            Self::PluginNotFound => write!(f, "plugin not found in the server"),
            Self::PaymentRequired => write!(f, "payment required for this plugin"),
            Self::Error(err) => write!(f, "{}", err),
        }
    }
}

impl Error for CliError {}

impl CliError {
    pub fn log(self) -> ! {
        eprintln!("\x1b[31macode cli:\x1b[0m {}", self);
        exit(1);
    }
}
