use crate::{cmd_exec::exec, error::CliError};

#[cfg(unix)]
pub fn open<U: Into<String>>(arg: U) -> Result<(), CliError> {
    exec("xdg-open", &[arg.into()])?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn open<U: Into<String>>(arg: U) -> Result<(), CliError> {
    exec("start", &[arg.into()])?;
    Ok(())
}
