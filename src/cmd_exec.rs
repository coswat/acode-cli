use crate::error::CliError;
use std::process::{Command, Stdio};

// pub struct Executer<'a> {
//     cmd: String,
//     args: Vec<&'a str>,
// }

pub fn exec<U>(cmd: &str, args: &[U]) -> Result<(), CliError>
where
    U: AsRef<str>,
{
    let mut cmd = Command::new(cmd);
    for arg in args {
        cmd.arg(arg.as_ref());
    }
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    let status = cmd.status().map_err(|e| CliError::Error(e.to_string()))?;
    if !status.success() {
        return Err(CliError::CommandFailed);
    }
    Ok(())
}
