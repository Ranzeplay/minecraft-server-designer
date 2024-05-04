use std::env;
use std::path::Path;

use crate::models::config::ModTargetSide;
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
}
