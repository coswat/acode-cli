use crate::command::Commands;
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct AcodeCli {
    #[clap(subcommand)]
    pub command: Commands,
}
