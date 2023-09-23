mod actions;
mod cli;
mod cmd_exec;
mod command;
mod config;
mod error;
mod open;

use crate::command::{Command, Commands};
use clap::Parser;
use cli::AcodeCli;

fn main() {
    let cmd = AcodeCli::parse();
    let rs = match cmd.command {
        Commands::Create(s) => s.action(),
        Commands::Docs(s) => s.action(),
        Commands::Build(s) => s.action(),
        Commands::BuildRelease(s) => s.action(),
        Commands::Src(s) => s.action(),
        Commands::Plugin(s) => s.action(),
        Commands::SelfUpdate(s) => s.action(),
    };
    match rs {
        Ok(_) => (),
        Err(e) => e.log(),
    }
}
