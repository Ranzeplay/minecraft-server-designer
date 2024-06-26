use crate::downloader::fabric_downloader::download_fabric_server;
use crate::downloader::vanilla_downloader::download_vanilla_server;
use crate::models::config::{ModLoader, ModTargetSide};
use crate::models::download_mod_metadata::DownloadModMetadata;
use crate::providers::modrinth_provider::ModrinthProvider;

#[tokio::test]
pub async fn modrinth_download_test() -> anyhow::Result<()> {
    let model = DownloadModMetadata {
        name: "Instant Marker".to_string(),
        mod_id: "instantmarker".to_string(),
        mod_version: "1.1".to_string(),
        mod_loader: ModLoader::Fabric,
        game_version: "1.19.3".to_string(),
        sides: vec![ModTargetSide::Server, ModTargetSide::Client],
    };

    ModrinthProvider::download_mod(model, false).await.unwrap();

    Ok(())
}

#[tokio::test]
pub async fn fabric_download_test() -> anyhow::Result<()> {
    download_fabric_server("1.19.3".to_string()).await
}

#[tokio::test]
pub async fn vanilla_download_test() -> anyhow::Result<()> {
    download_vanilla_server("1.19.3".to_string()).await
}
