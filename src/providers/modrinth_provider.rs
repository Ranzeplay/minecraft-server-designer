use colored::Colorize;
use crate::models::config::{AppConfig, ModMetadata, ModTargetSide};
use crate::models::download_mod_metadata::DownloadModMetadata;
use crate::models::download_result::{DownloadResult, DownloadStatus};
use crate::universal_downloader::download_file;

pub struct ModrinthProvider;

impl ModrinthProvider {
    pub async fn download_mod(metadata: DownloadModMetadata, force: bool) -> anyhow::Result<DownloadResult> {
        let response = reqwest::get(format!("https://api.modrinth.com/v2/project/{}/version?loaders=[\"{}\"]&game_versions=[\"{}\"]", metadata.mod_id, metadata.mod_loader.to_string(), metadata.game_version))
            .await?
            .json::<serde_json::Value>()
            .await?;

        for version_entry in response.as_array().unwrap() {
            if version_entry["version_number"].as_str().unwrap().eq(metadata.mod_version.as_str()) || metadata.mod_version.eq("latest") {
                for file_entry in version_entry["files"].as_array().unwrap() {
                    if file_entry["primary"].as_bool().unwrap() {
                        let download_url = file_entry["url"].as_str().unwrap();
                        let name = file_entry["filename"].as_str().unwrap();

                        let avail_text = format!("Mod {} is available", metadata.name);
                        println!("{}", avail_text.bright_blue());

                        let result = download_file(download_url, name, &metadata.sides, force).await?;
                        return Ok(DownloadResult::from_universal(&result, metadata.name));
                    }
                }
            }
        }

        let no_match_text = format!("No matching version for mod {}", metadata.name);
        println!("{}", no_match_text.red());

        Ok(DownloadResult {
            name: metadata.name.clone(),
            description: "Version mismatch",
            status: DownloadStatus::Failed,
        })
    }

    pub async fn get_mod_metadata(mut metadata: ModMetadata, config: &AppConfig) -> anyhow::Result<ModMetadata, String> {
        let response = match reqwest::get(format!("https://api.modrinth.com/v2/project/{}", metadata.mod_id)).await {
            Ok(resp) => {
                if !resp.status().is_success() {
                    return Err(format!("Failed to fetch mod metadata for mod ID {}: HTTP status code {}", 
                        metadata.mod_id, resp.status().as_u16()));
                }
                resp
            },
            Err(e) => return Err(format!("Failed to fetch mod metadata for mod ID {}: {}", metadata.mod_id, e)),
        };

        let response = match response.json::<serde_json::Value>().await {
            Ok(json) => json,
            Err(e) => return Err(format!("Failed to parse mod metadata for mod ID {}: {}", metadata.mod_id, e)),
        };
        
        let versions: Vec<String> = response["game_versions"].as_array().unwrap().iter().map(|version| version.as_str().unwrap().to_string()).collect();
        if !versions.contains(&config.game_version) {
            return Err("No matching version for mod".to_string());
        }
        
        let loaders: Vec<String> = response["loaders"].as_array().unwrap().iter().map(|loader| loader.as_str().unwrap().to_string()).collect();
        if !loaders.contains(&config.mod_loader.to_string().to_string()) {
            return Err("No matching loader for mod".to_string());
        }
        
        metadata.name = response["title"].as_str().unwrap().to_string();
        
        if response["client_side"].as_str().unwrap().ne("unsupported") {
            metadata.sides.push(ModTargetSide::Client);
        }
        if response["server_side"].as_str().unwrap().ne("unsupported") {
            metadata.sides.push(ModTargetSide::Server);
        }
        
        metadata.version = "latest".to_string();
        
        return Ok(metadata);
    }
}
