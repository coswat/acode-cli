use crate::cmd_exec::exec;
use anyhow::Result;

pub fn build() -> Result<()> {
    exec("npm", &["run", "build"])?;
    Ok(())
}

pub fn build_release() -> Result<()> {
    exec("npm", &["run", "build-release"])?;
    Ok(())
}
