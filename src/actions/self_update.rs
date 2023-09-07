use crate::command::Command;
use anyhow::{Error, Result};
use clap::Args;

#[derive(Debug, Args)]
pub struct SelfUpdate {}

impl Command for SelfUpdate {
    fn action(&self) -> Result<(), Error> {
        println!("Unimplemented! Check github for new release");
        Ok(())
    }
}
