use crate::downloader::modrinth_mod_downloader::{download_modrinth_mod, ModrinthModMetadata};
use crate::viewmodel::config::{AppConfig, ModProvider};

pub async fn build_all() -> anyhow::Result<()> {
    build_mods().await?;
    Ok(())
}

async fn build_mods() -> anyhow::Result<()> {
    let config = AppConfig::load();

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

                download_modrinth_mod(metadata).await?;
            }
            ModProvider::CurseForge => unimplemented!("Feature not implemented yet")
        }
    }

    Ok(())
}
