use crate::{cli::Command, error::CliError, open::open};
use clap::Args;

const SRC: &str = "https://github.com/coswat/acode-cli";

#[derive(Debug, Args)]
pub struct Src {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the source code url ( will not open )
    show: bool,
}

impl Command for Src {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        if self.show {
            show();
        } else {
            open(SRC)?;
        }
        Ok(())
    }
}

fn show() {
    println!("Acode Plugin CLI : {}", SRC);
}
