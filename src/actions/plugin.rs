use crate::{cli::Command, error::CliError, open::open};
use clap::Args;
use reqwest::blocking;
use serde::Deserialize;
use spinners::{Spinner, Spinners};
use std::{fs, io, process};

#[derive(Args, Debug)]
pub struct Plugin {
    /// Plugin ID
    #[arg(value_name("ID"))]
    name: String,
    /// Open the plugin url in web browser
    #[arg(short, long, default_value_t = false)]
    open: bool,
    /// Download the plugin zip
    #[arg(short, long, default_value_t = false)]
    download: bool,
}

#[derive(Debug, Deserialize)]
struct PluginData {
    id: String,
    name: String,
    price: u32,
    votes_up: u8,
    votes_down: u8,
    downloads: u32,
    author: String,
}

impl Command for Plugin {
    type Error = CliError;
    fn action(&self) -> Result<(), Self::Error> {
        if self.open {
            let url = format!("http://acode.foxdebug.com/plugin/{}", self.name);
            println!("Opening plugin url in browser");
            open(url)?;
            process::exit(0);
        }
        if self.download {
            self.download()?;
            process::exit(0);
        }
        let values = self.plugin_data()?;
        println!(" Id: {} \n Name: {} \n Price: {}₹ \n Likes: {} \n Dislikes: {} \n Downloads: {} \n Author: {} \n", values.id, values.name, values.price, values.votes_up, values.votes_down, values.downloads, values.author);
        Ok(())
    }
}

impl Plugin {
    fn plugin_data(&self) -> Result<PluginData, CliError> {
        let url = format!("http://acode.foxdebug.com/api/plugins/{}", self.name);
        let req = blocking::get(url.as_str()).map_err(|e| CliError::Error(e.to_string()))?;
        if req.status() == 404 {
            return Err(CliError::PluginNotFound);
        }
        let values = req
            .json::<PluginData>()
            .map_err(|e| CliError::Error(e.to_string()))?;
        Ok(values)
    }
    fn download(&self) -> Result<(), CliError> {
        let durl = format!(
            "http://acode.foxdebug.com/api/plugins/download/{}",
            self.name
        );
        let mut sp = Spinner::new(Spinners::Aesthetic, "Downloading".into());
        let mut req = blocking::get(durl).map_err(|e| CliError::Error(e.to_string()))?;
        if req.status() == 404 {
            sp.stop_with_message("".into());
            return Err(CliError::PluginNotFound);
        }
        if req.status() == 403 {
            sp.stop_with_message("".into());
            return Err(CliError::PaymentRequired);
        }
        let mut out = fs::File::create("plugin.zip").map_err(|e| CliError::Error(e.to_string()))?;
        io::copy(&mut req, &mut out).map_err(|e| CliError::Error(e.to_string()))?;
        sp.stop_with_message(" Plugin.zip installed successfully".to_string());
        Ok(())
    }
}
