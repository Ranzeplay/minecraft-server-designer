use std::path::PathBuf;
use std::sync::Mutex;
use clap::Parser;
use crate::init::init_all;
use crate::models::startup_args::{Commands, ModCommand, StartupArgs};

use anyhow::Result;
use java_locator::locate_java_home;
use lazy_static::lazy_static;
use crate::build_command::build_all;
use crate::mod_command::{add_mod, list_mods, remove_mod};

#[cfg(test)]
mod test;

mod models;
mod init;
mod downloader;
mod mod_command;
mod build_command;
mod providers;
mod universal_downloader;

lazy_static! {
    pub static ref CURSEFORGE_API_TOKEN: Mutex<String> = Mutex::new(String::new());
    pub static ref CONFIG_FILE_PATH: Mutex<PathBuf> = Mutex::new(PathBuf::new());
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = StartupArgs::parse();
    
    *CONFIG_FILE_PATH.lock().unwrap() = PathBuf::from(&args.config_path);

    // Check Java installation
    locate_java_home()
        .expect("Failed to locate Java");

    match args.command {
        Commands::Init(mut param) => {
            if param.game_version == "latest" {
                let versions = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
                    .await?
                    .json::<serde_json::Value>()
                    .await?;

                param.game_version = versions["latest"]["release"].as_str().unwrap().to_string();
            }

            init_all(param.game_version, param.mod_loader);
        }
        Commands::Mod(param) => {
            match param {
                ModCommand::Add(mod_to_add) => add_mod(mod_to_add).await?,
                ModCommand::List => list_mods(),
                ModCommand::Remove(mod_to_remove) => remove_mod(mod_to_remove.id),
            }
        }
        Commands::Build(x) => build_all(x.skip_server, x.force_download).await?
    }

    Ok(())
}
