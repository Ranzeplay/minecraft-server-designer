use colored::Colorize;
use crate::CURSEFORGE_API_TOKEN;
use crate::models::config::{AppConfig, ModMetadata, ModTargetSide};
use crate::models::download_mod_metadata::DownloadModMetadata;
use crate::models::download_result::{DownloadResult, DownloadStatus};
use crate::universal_downloader::download_file;

pub struct CurseForgeProvider;

impl CurseForgeProvider {
    pub async fn download_mod(
        metadata: DownloadModMetadata,
        force: bool,
    ) -> anyhow::Result<DownloadResult> {
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", reqwest::header::HeaderValue::from_str(&**CURSEFORGE_API_TOKEN.lock().unwrap()).unwrap());
        headers.insert("Accept", reqwest::header::HeaderValue::from_static("application/json"));

        let query = format!(
            "gameVersion={}&modLoaderType={}&index={}&pageSize={}",
            metadata.game_version,
            metadata.mod_loader.to_curseforge_id(),
            0,
            1
        );

        let response = client
            .get(format!(
                "https://api.curseforge.com/v1/mods/{}/files?{}",
                metadata.mod_id,
                query
            ))
            .headers(headers)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let entries = response["data"].as_array().unwrap();
        if entries.is_empty() {
            return Ok(DownloadResult {
                name: metadata.name.clone(),
                description: "No matching mod file found",
                status: DownloadStatus::Failed,
            });
        }

        let file_entry = entries.get(0).unwrap();

        let download_url = file_entry["downloadUrl"].as_str().unwrap();
        let name = file_entry["fileName"].as_str().unwrap();

        let avail_text = format!("Mod {} is available", metadata.name);
        println!("{}", avail_text.bright_blue());

        let result = download_file(download_url, name, &metadata.sides, force).await?;
        return Ok(DownloadResult::from_universal(&result, metadata.name));
    }

    pub async fn get_mod_metadata(mut metadata: ModMetadata, config: &AppConfig) -> anyhow::Result<ModMetadata, String> {
        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", reqwest::header::HeaderValue::from_str(&**CURSEFORGE_API_TOKEN.lock().unwrap()).unwrap());
        headers.insert("Accept", reqwest::header::HeaderValue::from_static("application/json"));

        let query = format!(
            "gameVersion={}&modLoaderType={}&index={}&pageSize={}",
            config.game_version,
            config.mod_loader.to_curseforge_id(),
            0,
            1
        );

        let project_response = client
            .get(format!(
                "https://api.curseforge.com/v1/mods/{}",
                metadata.mod_id,
            ))
            .headers(headers.clone())
            .send()
            .await
            .expect("Failed to fetch mod project metadata from CurseForge")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to parse mod project metadata from CurseForge");
        
        let file_response = client
            .get(format!(
                "https://api.curseforge.com/v1/mods/{}/files?{}",
                metadata.mod_id,
                query
            ))
            .headers(headers)
            .send()
            .await
            .expect("Failed to fetch mod file metadata from CurseForge")
            .json::<serde_json::Value>()
            .await
            .expect("Failed to parse mod file metadata from CurseForge");

        let entries = file_response["data"].as_array().unwrap();
        if entries.is_empty() {
            return Err("No matching mod found".to_string());
        }

        metadata.name = project_response["data"]["name"].as_str().unwrap().to_string();

        if metadata.sides.is_empty() {
            metadata.sides = vec![ModTargetSide::Client, ModTargetSide::Server];            
        }

        metadata.version = "latest".to_string();
        
        return Ok(metadata);
    }
}
