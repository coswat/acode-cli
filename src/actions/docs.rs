use crate::cmd_exec::exec;
use anyhow::{anyhow, Error, Result};
use std::env;

const DOC: &str = "https://acode.foxdebug.com/plugin-docs";

pub fn open() -> Result<(), Error> {
    match env::consts::OS {
        "linux" => exec("xdg-open", &[DOC])?,
        "macos" => exec("open", &[DOC])?,
        "windows" => exec("start", &[DOC])?,
        "android" => exec("xdg-open", &[DOC])?,
        _ => {
            Err::<&str, Error>(anyhow!("Platform not supported"));
            return Ok(());
        }
    }
    Ok(())
}

pub fn show() {
    println!("Acode Plugun Docs: {}", DOC);
}
