use crate::{cmd_exec::exec, command::Command, error::CliError};
use clap::Args;

const DOC: &str = "https://acode.foxdebug.com/plugin-docs";

#[derive(Debug, Args)]
pub struct Docs {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the docs url ( will not open )
    show: bool,
}

impl Command for Docs {
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
    println!("Acode Plugun Docs: {}", DOC);
}

#[cfg(target_os = "android")]
fn open() -> Result<(), CliError> {
    exec("xdg-open", &[DOC])?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn open() -> Result<(), CliError> {
    exec("xdg-open", &[DOC])?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn open() -> Result<(), CliError> {
    exec("open", &[DOC])?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn open() -> Result<(), CliError> {
    exec("start", &[DOC])?;
    Ok(())
}
