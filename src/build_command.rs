use colored::Colorize;
use tokio::task;

use crate::downloader::fabric_downloader::download_fabric_server;
use crate::downloader::modrinth_mod_downloader::{download_modrinth_mod, ModrinthModMetadata};
use crate::downloader::vanilla_downloader::download_vanilla_server;
use crate::viewmodel::config::{AppConfig, ModLoader, ModProvider};
use crate::viewmodel::download_result::DownloadStatus;

pub async fn build_all(skip_server: bool, force_mods: bool) -> anyhow::Result<()> {
    build_mods(force_mods).await?;
    if !skip_server {
        build_server().await?;
    } else {
        println!("{}", "Server files download is skipped".bold());
    }

    println!("{}", "Operation completed".bold());
    Ok(())
}

pub async fn build_server() -> anyhow::Result<()> {
    let config = AppConfig::load();

    match config.mod_loader {
        ModLoader::Vanilla => download_vanilla_server(config.game_version).await?,
        ModLoader::Fabric => download_fabric_server(config.game_version).await?,
        _ => panic!("Unsupported mod loader")
    }

    Ok(())
}

async fn build_mods(force: bool) -> anyhow::Result<()> {
    let config = AppConfig::load();

    println!("{}", "Downloading mods".bold());
    let mut handles = vec![];
    for mc_mod in config.mods {
        let game_version = config.game_version.clone();
        let mod_loader = config.mod_loader.clone();

        match mc_mod.provider {
            ModProvider::Modrinth => {
                let metadata = ModrinthModMetadata {
                    name: mc_mod.name,
                    mod_id: mc_mod.mod_id,
                    mod_version: mc_mod.version,
                    mod_loader,
                    game_version,
                    sides: mc_mod.sides,
                };
                handles.push(task::spawn(download_modrinth_mod(metadata, force)));
            }
            ModProvider::CurseForge => unimplemented!("Feature not implemented yet")
        }
    }

    let mut results = vec![];
    for handle in handles {
        let result = handle.await?;
        results.push(result.expect("Unexpected error when unpacking download result"));
    }

    println!("{}", "Summary".bold());
    println!("Total: {} | Downloaded: {} | Failed: {} | Skipped: {}",
             results.len(),
             results.iter().filter(|r| r.status == DownloadStatus::Downloaded).count(),
             results.iter().filter(|r| r.status == DownloadStatus::Failed).count(),
             results.iter().filter(|r| r.status == DownloadStatus::Skipped).count());
    for failed_mod in results.iter().filter(|r| r.status == DownloadStatus::Failed) {
        println!("{}: {}", failed_mod.name.red(), failed_mod.description);
    }

    Ok(())
}
