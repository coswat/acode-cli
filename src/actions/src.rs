use crate::cmd_exec::exec;
use anyhow::{anyhow, Error, Result};
use std::env;
const SRC: &str = "https://github.com/coswat/acode-cli";

pub fn open() -> Result<(), Error> {
    match env::consts::OS {
        "linux" => exec("xdg-open", &[SRC])?,
        "macos" => exec("open", &[SRC])?,
        "windows" => exec("start", &[SRC])?,
        "android" => exec("xdg-open", &[SRC])?,
        _ => {
            Err::<&str, Error>(anyhow!("Platform not supported"));
            return Ok(());
        }
    }
    Ok(())
}

pub fn show() {
    println!("Acode Plugin CLI : {}", SRC);
}
