use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginJson {
    pub id: String,
    pub name: String,
    pub main: String,
    pub version: String,
    pub readme: String,
    pub icon: String,
    pub files: Vec<String>,
    #[serde(rename = "minVersionCode")]
    pub min_version_code: i32,
    pub price: i32,
    pub author: Author,
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub github: String,
}

#[derive(Debug)]
pub struct Answers {
    pub name: String,
    pub lang: String,
    pub prettier: bool,
    pub git: bool,
    pub plugin_id: String,
    pub price: i32,
    pub author: String,
    pub email: String,
    pub github_name: String,
    pub install_dep: bool,
}

pub fn prompts() -> Answers {
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
    let prettier = Confirm::with_theme(&theme)
        .with_prompt("Do you want to use Prettier for code formatting?")
        .default(true)
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
        prettier,
        git,
        plugin_id,
        price,
        author,
        email,
        github_name,
        install_dep,
    }
}

pub fn plugin_json(
    id: String,
    price: i32,
    name: String,
    email: String,
    github: String,
) -> Result<()> {
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
