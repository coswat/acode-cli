use crate::cli::Command;
use crate::error::CliError;
use clap::Args;

#[derive(Debug, Args)]
pub struct SelfUpdate {}

impl Command for SelfUpdate {
    type Error = CliError;
    fn action(&self) -> Result<(), CliError> {
        println!("Unimplemented! Check github for new release");
        Ok(())
    }
}
