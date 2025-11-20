use crate::actions::{
    build::{Build, BuildRelease},
    create::Create,
    docs::Docs,
    plugin::Plugin,
    self_update::SelfUpdate,
    src::Src,
};
use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
pub struct AcodeCli {
    #[clap(subcommand)]
    pub command: Commands,
}

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a acode plugin template
    Create(Create),
    /// Open the plugin docs
    Docs(Docs),
    /// Alternative of npm run build
    Build(Build),
    /// Alternative of npm run build-release
    BuildRelease(BuildRelease),
    /// Acode cli source code
    Src(Src),
    /// Do plugin details search and details
    Plugin(Plugin),
    /// Update the Acode cli to the latest version
    SelfUpdate(SelfUpdate),
}

pub trait Command {
    type Error;
    fn action(&self) -> Result<(), Self::Error>;
}
