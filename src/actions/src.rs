use crate::{cmd_exec::exec, command::Command, error::CliError};
use clap::Args;

const SRC: &str = "https://github.com/coswat/acode-cli";

#[derive(Debug, Args)]
pub struct Src {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the source code url ( will not open )
    show: bool,
}

impl Command for Src {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        if self.show {
            show();
        } else {
            open()?;
        }
        Ok(())
    }
}

fn show() {
    println!("Acode Plugin CLI : {}", SRC);
}

#[cfg(target_os = "android")]
fn open() -> Result<(), CliError> {
    exec("xdg-open", &[SRC])?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn open() -> Result<(), CliError> {
    exec("xdg-open", &[SRC])?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn open() -> Result<(), CliError> {
    exec("open", &[SRC])?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn open() -> Result<(), CliError> {
    exec("start", &[SRC])?;
    Ok(())
}
