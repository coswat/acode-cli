use crate::actions::{
    build::{Build, BuildRelease},
    create::Create,
    docs::Docs,
    self_update::SelfUpdate,
    src::Src,
};
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
    /// Update the Acode cli to the latest version
    SelfUpdate(SelfUpdate),
}

pub trait Command {
    type Error;
    fn action(&self) -> Result<(), Self::Error>;
}
