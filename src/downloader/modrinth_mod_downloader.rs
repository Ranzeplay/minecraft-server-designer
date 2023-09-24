use std::env;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use crate::viewmodel::config::{ModLoader, ModTargetSide};

pub struct ModrinthModMetadata {
    pub name: String,
    pub mod_id: String,
    pub mod_version: String,
    pub mod_loader: ModLoader,
    pub game_version: String,
    pub sides: Vec<ModTargetSide>
}
pub async fn download_modrinth_mod(metadata: ModrinthModMetadata) -> anyhow::Result<()> {
    println!("Downloading {} from Modrinth", metadata.name);
    let response = reqwest::get(format!("https://api.modrinth.com/v2/project/{}/version?loaders=[\"{}\"]&game_versions=[\"{}\"]", metadata.mod_id, metadata.mod_loader.to_string(), metadata.game_version))
        .await?
        .json::<serde_json::Value>()
        .await?;

    for version_entry in response.as_array().unwrap() {
        if version_entry["version_number"].as_str().unwrap().eq(metadata.mod_version.as_str()) {
            for file_entry in version_entry["files"].as_array().unwrap() {
                if file_entry["primary"].as_bool().unwrap() {
                    let download_url = file_entry["url"].as_str().unwrap();
                    let name = file_entry["filename"].as_str().unwrap();

                    println!("Mod {} is available", metadata.name);

                    // Download files
                    let file_content = reqwest::get(download_url)
                        .await?
                        .bytes()
                        .await?;
                    let download_path = &env::current_dir().unwrap()
                        .join("mods")
                        .join("downloading")
                        .join(name);

                    println!("Writing mod file of {} to local", metadata.name);
                    let mut file = File::create(download_path).await?;
                    file.write_all(&file_content).await?;

                    if metadata.sides.contains(&ModTargetSide::Client) {
                        let dest = env::current_dir().unwrap()
                            .join("mods")
                            .join("client")
                            .join(name);
                        tokio::fs::copy(download_path, dest).await?;
                    }

                    if metadata.sides.contains(&ModTargetSide::Server) {
                        let dest = env::current_dir().unwrap()
                            .join("mods")
                            .join("server")
                            .join(name);
                        tokio::fs::copy(download_path, dest).await?;
                    }

                    tokio::fs::remove_file(download_path).await?;

                    return Ok(());
                }
            }
        }
    }

    println!("No matching version for mod {}", metadata.name);

    Ok(())
}
