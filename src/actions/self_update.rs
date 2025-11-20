use crate::{cli::Command, error::CliError};
use clap::Args;

#[derive(Debug, Args)]
pub struct SelfUpdate {}

impl Command for SelfUpdate {
    fn action(&self) -> Result<(), CliError> {
        println!("Unimplemented! Check src url (github) for new release >_<");
        Ok(())
    }
}
