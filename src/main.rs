mod actions;
mod cli;
mod cmd_exec;
mod command;
mod config;

use crate::command::{Command, Commands};
use anyhow::{Error, Result};
use clap::Parser;
use cli::AcodeCli;

fn main() -> Result<(), Error> {
    let cmd = AcodeCli::parse();
    match cmd.command {
        Commands::Create(s) => s.action()?,
        Commands::Docs(s) => s.action()?,
        Commands::Build(s) => s.action()?,
        Commands::BuildRelease(s) => s.action()?,
        Commands::Src(s) => s.action()?,
        Commands::SelfUpdate(s) => s.action()?,
    };
    Ok(())
}
