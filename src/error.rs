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
    DependencyNotFound,
    Error(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        match self {
            Self::CommandFailed => write!(f, "Command failed"),
            Self::PluginNotFound => write!(f, "Plugin not found in the server"),
            Self::PaymentRequired => write!(f, "Payment is required for this plugin"),
            Self::DependencyNotFound => write!(
                f,
                "The program for this command is not installed in your device"
            ),
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
