use clap::Parser;
use crate::init::init_all;
use crate::viewmodel::startup_args::{Commands, StartupArgs};

use anyhow::Result;

mod viewmodel;
mod init;
mod downloader;
#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> Result<()> {
    let args = StartupArgs::parse();

    match args.command {
        Commands::Init(mut param) => {
            if param.game_version == "latest" {
                let versions = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
                    .await?
                    .json::<serde_json::Value>()
                    .await?;

                param.game_version = versions["latest"]["release"].as_str().unwrap().to_string();
            }


            init_all(param.game_version, param.client_mod_loader, param.server_mod_loader);
        }
    }

    println!("Hello, world!");

    Ok(())
}
