use crate::{cli::Command, error::CliError, open::open};
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
    fn action(&self) -> Result<(), CliError> {
        if self.show {
            show();
        } else {
            println!("Trying to open the docs url in a browser");
            open(DOC)?;
        }
        Ok(())
    }
}

fn show() {
    println!("Acode Plugun Docs: {}", DOC);
}
