use crate::{cmd_exec::exec, command::Command, error::CliError};
use anyhow::{Error, Result};
use clap::Args;
use std::env;

const SRC: &str = "https://github.com/coswat/acode-cli";

#[derive(Debug, Args)]
pub struct Src {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the source code url ( will not open )
    show: bool,
}

impl Command for Src {
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
        "linux" => exec("xdg-open", &[SRC])?,
        "macos" => exec("open", &[SRC])?,
        "windows" => exec("start", &[SRC])?,
        "android" => exec("xdg-open", &[SRC])?,
        _ => {
            Err::<&str, CliError>(CliError::PlatformNotSupported)?;
            return Ok(());
        }
    }
    Ok(())
}

fn show() {
    println!("Acode Plugin CLI : {}", SRC);
}
