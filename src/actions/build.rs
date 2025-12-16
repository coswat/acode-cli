use crate::{cli::Command, error::CliError, helpers::exec};
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
    fn action(&self) -> Result<(), CliError> {
        if self.release {
            build_release()?;
        } else {
            build()?;
        }
        Ok(())
    }
}

impl Command for BuildRelease {
    fn action(&self) -> Result<(), CliError> {
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
