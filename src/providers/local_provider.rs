use std::env;
use std::path::Path;

use crate::models::config::{AppConfig, ModMetadata, ModProvider, ModTargetSide};
use crate::models::download_mod_metadata::DownloadModMetadata;
use crate::models::download_result::{DownloadResult, DownloadStatus};

pub struct LocalProvider;

impl LocalProvider {
    pub async fn download_mod(metadata: DownloadModMetadata, _force: bool) -> anyhow::Result<DownloadResult> {
        let file_result = Path::new(&metadata.mod_id);
        if file_result.exists() {
            let filename = file_result.file_name().unwrap();
            if metadata.sides.contains(&ModTargetSide::Client) {
                let dest = env::current_dir().unwrap()
                    .join("mods")
                    .join("client")
                    .join(filename);
                tokio::fs::copy(file_result, dest).await?;
            }

            if metadata.sides.contains(&ModTargetSide::Server) {
                let dest = env::current_dir().unwrap()
                    .join("mods")
                    .join("server")
                    .join(filename);
                tokio::fs::copy(file_result, dest).await?;
            }
            
            Ok(DownloadResult {
                name: metadata.name.clone(),
                description: "Successfully downloaded mod file",
                status: DownloadStatus::Downloaded,
            })
        } else {
            Ok(DownloadResult {
                name: metadata.name.clone(),
                description: "Mod not found",
                status: DownloadStatus::Failed,
            })
        }
    }

    pub async fn get_mod_metadata(mut metadata: ModMetadata, _config: &AppConfig) -> anyhow::Result<ModMetadata, String> {
        let file_result = Path::new(&metadata.mod_id);
        if file_result.exists() {
            metadata.name = file_result.file_name().unwrap().to_str().unwrap().to_string();
            metadata.sides = vec![ModTargetSide::Client, ModTargetSide::Server];
            metadata.version = "latest".to_string();
            metadata.provider = ModProvider::Local;
            Ok(metadata)
        } else {
            Err("Mod not found".to_string())
        }
    }
}
