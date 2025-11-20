use crate::{cli::Command, cmd_exec::exec, error::CliError};
use clap::Args;

#[derive(Debug, Args)]
pub struct Build {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Build release argument
    release: bool,
}

#[derive(Debug, Args)]
pub struct BuildRelease {}

impl Command for Build {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        if self.release {
            build_release()?;
        } else {
            build()?;
        }
        Ok(())
    }
}

impl Command for BuildRelease {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        build_release()?;
        Ok(())
    }
}

fn build() -> Result<(), CliError> {
    exec("npm", &["run", "build"])?;
    Ok(())
}

fn build_release() -> Result<(), CliError> {
    exec("npm", &["run", "build-release"])?;
    Ok(())
}
