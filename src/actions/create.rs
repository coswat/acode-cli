use crate::{cmd_exec::exec, command::Command, config::Config};
use anyhow::{Error, Result};
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
    lang: String,
    git: bool,
    plugin_id: String,
    price: i32,
    author: String,
    email: String,
    github_name: String,
    install_dep: bool,
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
    fn action(&self) -> Result<(), Error> {
        let config = Config::new();
        let ans = prompts();
        let dir = env::current_dir()?;
        env::set_current_dir(&dir)?;
        let mut sp = Spinner::new(Spinners::Line, "Cloning plugin template...".into());
        if ans.lang == "JavaScript" {
            exec("git", &["clone", config.js_template, &ans.name])?;
        } else {
            exec("git", &["clone", config.ts_template, &ans.name])?;
        }
        sp.stop_with_message(" Plugin template cloned successfully".into());
        env::set_current_dir(dir.join(&ans.name))?;
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
            let mut sp = Spinner::new(Spinners::Line, "Installing npm dependencies...".into());
            exec("npm", &["install"])?;
            sp.stop_with_message(" Installed npm dependencies".into());
        }
        println!(" Plugin created successfully");
        Ok(())
    }
}

fn prompts() -> Answers {
    let theme = ColorfulTheme::default();
    let name = Input::with_theme(&theme)
        .with_prompt("Enter a name for your Acode plugin:")
        .default("test-plugin".to_string())
        .interact_text()
        .unwrap();
    let langs = &["JavaScript", "TypeScript"];
    let lang = Select::with_theme(&theme)
        .with_prompt("Choose a language")
        .default(0)
        .items(&langs[..])
        .interact()
        .unwrap();
    let git = Confirm::with_theme(&theme)
        .with_prompt("Initializing Git repository")
        .default(true)
        .interact()
        .unwrap();
    let plugin_id = Input::with_theme(&theme)
        .with_prompt("Enter id for your Acode Plugin:")
        .default("test_plugin_12".to_string())
        .interact()
        .unwrap();
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
        .unwrap();
    let author = Input::with_theme(&theme)
        .with_prompt("Enter the Name of Plugin developer:")
        .default("".to_string())
        .interact()
        .unwrap();
    let email = Input::with_theme(&theme)
        .with_prompt("Enter the Email of Plugin developer:")
        .default("".to_string())
        .interact()
        .unwrap();
    let github_name = Input::with_theme(&theme)
        .with_prompt("Enter the Github username of Plugin developer!:")
        .default("".to_string())
        .interact()
        .unwrap();
    let install_dep = Confirm::with_theme(&theme)
        .with_prompt("Install npm dependencies?")
        .default(true)
        .interact()
        .unwrap();

    Answers {
        name,
        lang: langs[lang].to_string(),
        git,
        plugin_id,
        price,
        author,
        email,
        github_name,
        install_dep,
    }
}

fn plugin_json(id: String, price: i32, name: String, email: String, github: String) -> Result<()> {
    let mut json = PluginJson::default();
    json.id = id;
    json.price = price;
    json.author.name = name;
    json.author.email = email;
    json.author.github = github;
    let json_str = serde_json::to_string_pretty(&json)?;
    fs::write("plugin.json", json_str)?;
    Ok(())
}
