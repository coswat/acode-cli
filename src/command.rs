use crate::{cmd_exec::exec, config::Config};
use anyhow::{Result, Error};
use clap::{Args, Subcommand};
use std::env;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a acode plugin template
    Create(Create),
    /// Open the plugin docs
    Docs(Docs),
    /// Alternative of npm run build
    Build(Build),
    /// Alternative of npm run build-release
    BuildRelease(BuildRelease),
    /// Acode cli source code
    Src(Src),
    /// Update the Acode cli to the latest version
    SelfUpdate(SelfUpdate),
}

pub trait Command {
    fn action(&self) -> Result<(), Error>;
}

#[derive(Debug, Args)]
pub struct Create {}

#[derive(Debug, Args)]
pub struct Docs {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the docs url ( will not open )
    show: bool,
}

#[derive(Debug, Args)]
pub struct Build {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Build release argument
    release: bool,
}

#[derive(Debug, Args)]
pub struct BuildRelease {}

#[derive(Debug, Args)]
pub struct Src {
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    /// Show the source code url ( will not open )
    show: bool,
}

#[derive(Debug, Args)]
pub struct SelfUpdate {}

impl Command for Create {
    fn action(&self) -> Result<(), Error> {
        use crate::actions::create;
        let config = Config::new();
        let ans = create::prompts();
        let dir = env::current_dir()?;
        env::set_current_dir(&dir)?;
        if ans.lang == "JavaScript" {
            exec("git", &["clone", config.js_template, &ans.name])?;
        } else {
            exec("git", &["clone", config.ts_template, &ans.name])?;
        }
        env::set_current_dir(dir.join(&ans.name))?;
        exec("rm", &["-rf", ".git"])?;
        exec("rm", &["-rf", "plugin.json"])?;
        if ans.git {
            exec("git", &["init"])?;
            exec("git", &["add", "."])?;
            exec("git", &["commit", "-m", "Initial commit"])?;
        }
        create::plugin_json(
            ans.plugin_id,
            ans.price,
            ans.author,
            ans.email,
            ans.github_name,
        )?;
        if ans.install_dep {
            exec("npm", &["install"])?;
        }
        Ok(())
    }
}

impl Command for Docs {
    fn action(&self) -> Result<(), Error> {
        use crate::actions::docs;
        if self.show {
            docs::show();
        } else {
            docs::open()?;
        }
        Ok(())
    }
}

impl Command for Build {
    fn action(&self) -> Result<(), Error> {
        use crate::actions::build;
        if self.release {
            build::build_release()?;
        } else {
            build::build()?;
        }
        Ok(())
    }
}

impl Command for BuildRelease {
    fn action(&self) -> Result<(), Error> {
        use crate::actions::build;
        build::build_release()?;
        Ok(())
    }
}

impl Command for Src {
    fn action(&self) -> Result<(), Error> {
        use crate::actions::src;
        if self.show {
            src::show();
        } else {
            src::open()?;
        }
        Ok(())
    }
}

impl Command for SelfUpdate {
    fn action(&self) -> Result<(), Error> {
        println!("success");
        Ok(())
    }
}
