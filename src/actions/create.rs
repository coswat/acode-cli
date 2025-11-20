use crate::{cli::Command, cmd_exec::exec, config::Config, error::CliError};
use clap::Args;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use serde_json;
use spinners::{Spinner, Spinners};
use std::{env, fs};

#[derive(Debug, Args)]
pub struct Create {}

#[derive(Debug, Serialize, Deserialize)]
struct PluginJson {
    id: String,
    name: String,
    main: String,
    version: String,
    readme: String,
    icon: String,
    files: Vec<String>,
    #[serde(rename = "minVersionCode")]
    min_version_code: i32,
    price: i32,
    author: Author,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Author {
    name: String,
    email: String,
    github: String,
}

#[derive(Debug)]
struct Answers {
    name: String,
    lang: i32,
    git: bool,
    plugin_id: String,
    price: i32,
    author: String,
    email: String,
    github_name: String,
    install_dep: bool,
    package_manager: Option<String>,
}

impl Default for PluginJson {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            name: "Plugin".to_string(),
            main: "dist/main.js".to_string(),
            version: "1.0.0".to_string(),
            readme: "readme.md".to_string(),
            icon: "icon.png".to_string(),
            files: Vec::new(),
            min_version_code: 290,
            price: 0,
            author: Author::default(),
        }
    }
}

impl Command for Create {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        let config = Config::new();
        let ans = prompts()?;
        let dir = env::current_dir().map_err(|e| CliError::Error(e.to_string()))?;
        env::set_current_dir(&dir).map_err(|e| CliError::Error(e.to_string()))?;
        let mut sp = Spinner::new(Spinners::Line, "Cloning plugin template...".into());
        // 0 - JavaScript
        if ans.lang == 0 {
            exec("git", &["clone", config.js_template, &ans.name])?;
        } else {
            exec("git", &["clone", config.ts_template, &ans.name])?;
        }
        sp.stop_with_message(" Plugin template cloned successfully".into());
        env::set_current_dir(dir.join(&ans.name)).map_err(|e| CliError::Error(e.to_string()))?;
        exec("rm", &["-rf", ".git"])?;
        exec("rm", &["-rf", "plugin.json"])?;
        if ans.git {
            let mut sp = Spinner::new(Spinners::Line, "Initializing git repo...".into());
            exec("git", &["init"])?;
            exec("git", &["add", "."])?;
            exec("git", &["commit", "-m", "Initial commit"])?;
            sp.stop_with_message(" Initialized git repo".into());
        }
        let mut sp = Spinner::new(Spinners::Line, "Creating plugin.json file".into());
        plugin_json(
            ans.plugin_id,
            ans.price,
            ans.author,
            ans.email,
            ans.github_name,
        )?;
        sp.stop_with_message(" Created plugin.json".into());
        if ans.install_dep {
            let mut sp = Spinner::new(Spinners::Line, "Installing dependencies...".into());
            let pm = ans.package_manager.unwrap();
            exec(pm.as_str(), &["install"])?;
            sp.stop_with_message(" Installed dependencies".into());
        }
        println!(" Plugin created successfully");
        Ok(())
    }
}

fn prompts() -> Result<Answers, CliError> {
    let theme = ColorfulTheme::default();
    let name = Input::with_theme(&theme)
        .with_prompt("Enter a name for your Acode plugin:")
        .default("test-plugin".to_string())
        .interact_text()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let langs = &["JavaScript", "TypeScript"];
    let lang = Select::with_theme(&theme)
        .with_prompt("Choose a language")
        .default(0)
        .items(&langs[..])
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let git = Confirm::with_theme(&theme)
        .with_prompt("Initializing Git repository")
        .default(true)
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let plugin_id = Input::with_theme(&theme)
        .with_prompt("Enter id for your Acode Plugin:")
        .default("test_plugin_12".to_string())
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let price = Input::with_theme(&theme)
        .with_prompt("Enter price for your Acode Plugin in INR, if it's free then leave it on default value:")
        .default(0)
        .validate_with(|inp: &i32| -> Result<(), &str> {
         let cond = *inp == 0 || *inp > 10 && *inp < 10000;
            if !cond {
               return Err("Amount should be between 10 - 10000 INR");
            }
            Ok(())
        })
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let author = Input::with_theme(&theme)
        .with_prompt("Enter the Name of Plugin developer:")
        .default("".to_string())
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let email = Input::with_theme(&theme)
        .with_prompt("Enter the Email of Plugin developer:")
        .default("".to_string())
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let github_name = Input::with_theme(&theme)
        .with_prompt("Enter the Github username of Plugin developer!:")
        .default("".to_string())
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let install_dep = Confirm::with_theme(&theme)
        .with_prompt("Install dependencies?")
        .default(true)
        .interact()
        .map_err(|e| CliError::Error(e.to_string()))?;
    let pms = &["npm", "pnpm", "yarn", "bun"];
    let mut pm = 0;
    if install_dep {
        pm = Select::with_theme(&theme)
            .with_prompt("Choose a package manager")
            .default(0)
            .items(&pms[..])
            .interact()
            .map_err(|e| CliError::Error(e.to_string()))?;
    }

    let mut ans = Answers {
        name,
        lang: lang as i32,
        git,
        plugin_id,
        price,
        author,
        email,
        github_name,
        install_dep,
        package_manager: None,
    };
    if install_dep {
        ans.package_manager = Some(pms[pm].to_string());
    }

    Ok(ans)
}

fn plugin_json(
    id: String,
    price: i32,
    name: String,
    email: String,
    github: String,
) -> Result<(), CliError> {
    let json = PluginJson {
        id,
        price,
        author: Author {
            name,
            email,
            github,
        },
        ..Default::default()
    };
    let json_str =
        serde_json::to_string_pretty(&json).map_err(|e| CliError::Error(e.to_string()))?;
    fs::write("plugin.json", json_str).map_err(|e| CliError::Error(e.to_string()))?;
    Ok(())
}
