use colored::Colorize;
use crate::CURSEFORGE_API_TOKEN;

use crate::downloader::universal_downloader::download_file;
use crate::models::download_mod_metadata::DownloadModMetadata;
use crate::models::download_result::{DownloadResult, DownloadStatus};

pub async fn download_curseforge_mod(
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
