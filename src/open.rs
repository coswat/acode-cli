use crate::{cmd_exec::exec, error::CliError};

#[cfg(target_os = "android")]
pub fn open<U: Into<String>>(arg: U) -> Result<(), CliError> {
    exec("xdg-open", &[arg.into()])?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn open<U: Into<String>>(arg: U) -> Result<(), CliError> {
    exec("xdg-open", &[arg.into()])?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn open<U: Into<String>>(arg: U) -> Result<(), CliError> {
    exec("open", &[arg.into()])?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn open<U: Into<String>>(arg: U) -> Result<(), CliError> {
    exec("start", &[arg.into()])?;
    Ok(())
}
