use crate::{cmd_exec::exec, command::Command, error::CliError};
use anyhow::{Error, Result};
use clap::Args;
use std::env;

const DOC: &str = "https://acode.foxdebug.com/plugin-docs";

#[derive(Debug, Args)]
pub struct Docs {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the docs url ( will not open )
    show: bool,
}

impl Command for Docs {
    fn action(&self) -> Result<(), Error> {
        if self.show {
            show();
        } else {
            open()?;
        }
        Ok(())
    }
}

fn open() -> Result<(), Error> {
    match env::consts::OS {
        "linux" => exec("xdg-open", &[DOC])?,
        "macos" => exec("open", &[DOC])?,
        "windows" => exec("start", &[DOC])?,
        "android" => exec("xdg-open", &[DOC])?,
        _ => {
            Err::<&str, CliError>(CliError::PlatformNotSupported)?;
            return Ok(());
        }
    }
    Ok(())
}

fn show() {
    println!("Acode Plugun Docs: {}", DOC);
}
