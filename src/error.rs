use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Platform not supported")]
    PlatformNotSupported,
    #[error("Error executing command")]
    CommandFailed,
    //  #[error("Unknown error occurred")]
    //  Unknown,
}
