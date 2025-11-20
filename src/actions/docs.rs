use crate::{cli::Command, cmd_exec::exec, error::CliError, open::open};
use clap::Args;

const DOC: &str = "https://docs.acode.app/";

#[derive(Debug, Args)]
pub struct Docs {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the docs url ( will not open )
    show: bool,
}

impl Command for Docs {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        if self.show {
            show();
        } else {
            open(DOC)?;
        }
        Ok(())
    }
}

fn show() {
    println!("Acode Plugun Docs: {}", DOC);
}
