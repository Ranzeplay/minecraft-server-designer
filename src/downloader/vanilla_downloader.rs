use std::env;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_vanilla_server(game_version: String) -> anyhow::Result<()> {
    println!("Downloading vanilla server");
    let versions_response = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
        .await?
        .json::<Value>()
        .await?;

    let versions = versions_response["versions"].as_array().unwrap();
    for version in versions {
        if version["id"].as_str().unwrap().eq(&*game_version) {
            // Get metadata of target version
            println!("Acquiring version metadata");
            let version_response = reqwest::get(version["url"].as_str().unwrap())
                .await?
                .json::<Value>()
                .await?;

            println!("Downloading server JAR package");
            let server_url = version_response["downloads"]["server"]["url"].as_str().unwrap();
            let file_content = reqwest::get(server_url)
                .await?
                .bytes()
                .await?;
            let server_file_path = &env::current_dir().unwrap().join("server").join("server.jar");
            let mut file = File::create(server_file_path).await?;
            file.write_all(&file_content).await?;

            return Ok(())
        }
    }

    println!("Target game version does not found");
    Ok(())
}
