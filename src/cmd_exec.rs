use crate::error::CliError;
use std::process::{Command, Stdio};

pub fn exec<U: AsRef<str>>(cmd: &str, args: &[U]) -> Result<(), CliError> {
    let mut cmd = Command::new(cmd);
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    let status = cmd.status().map_err(|_| CliError::DependencyNotFound)?;
    if !status.success() {
        return Err(CliError::CommandFailed);
    }
    Ok(())
}
