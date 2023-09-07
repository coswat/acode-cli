use crate::error::CliError;
use anyhow::Result;
use std::process::{Command, Stdio};

// pub struct Executer<'a> {
//     cmd: String,
//     args: Vec<&'a str>,
// }

pub fn exec<T, U>(cmd: T, args: &[U]) -> Result<()>
where
    T: Into<String>,
    U: AsRef<str>,
{
    let mut cmd = Command::new(cmd.into());
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    let status = cmd.status()?;
    if !status.success() {
        return Err(CliError::CommandFailed.into());
    }
    Ok(())
}
