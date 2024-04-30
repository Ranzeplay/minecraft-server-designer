use colored::Colorize;

use crate::downloader::universal_downloader::download_file;
use crate::viewmodel::download_mod_metadata::DownloadModMetadata;
use crate::viewmodel::download_result::{DownloadResult, DownloadStatus};

pub async fn download_modrinth_mod(metadata: DownloadModMetadata, force: bool) -> anyhow::Result<DownloadResult> {
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
