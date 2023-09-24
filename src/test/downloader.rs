use crate::downloader::modrinth_mod_downloader::{download_modrinth_mod, ModrinthModMetadata};
use crate::viewmodel::config::{ModLoader, ModTargetSide};

#[tokio::test]
pub async fn modrinth_download_test() -> anyhow::Result<()> {
    let model = ModrinthModMetadata {
        name: "Instant Marker".to_string(),
        mod_id: "instantmarker".to_string(),
        mod_version: "1.1".to_string(),
        mod_loader: ModLoader::Fabric,
        game_version: "1.19.3".to_string(),
        sides: vec![ModTargetSide::Server, ModTargetSide::Client],
    };

    download_modrinth_mod(model).await
}