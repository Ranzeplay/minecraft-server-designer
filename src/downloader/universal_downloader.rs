use std::env;

use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::viewmodel::config::ModTargetSide;
use crate::viewmodel::download_result::DownloadStatus;

#[derive(Debug, Copy, Clone)]
pub struct UniversalDownloadResult {
    pub status: DownloadStatus,
    pub description: &'static str,
}

pub async fn download_file(url: &str, filename: &str, sides: &Vec<ModTargetSide>, force: bool) -> anyhow::Result<UniversalDownloadResult> {
    if check_availability(sides, filename) && !force {
        return Ok(UniversalDownloadResult {
            status: DownloadStatus::Skipped,
            description: "Skipped due to existing file",
        });
    }

    let download_path = &env::current_dir().unwrap()
                                           .join("mods")
                                           .join("downloading")
                                           .join(filename);
    let file_content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let mut file = File::create(download_path).await?;
    file.write_all(&file_content).await?;

    if sides.contains(&ModTargetSide::Client) {
        let dest = env::current_dir().unwrap()
                                     .join("mods")
                                     .join("client")
                                     .join(filename);
        tokio::fs::copy(download_path, dest).await?;
    }

    if sides.contains(&ModTargetSide::Server) {
        let dest = env::current_dir().unwrap()
                                     .join("mods")
                                     .join("server")
                                     .join(filename);
        tokio::fs::copy(download_path, dest).await?;
    }

    tokio::fs::remove_file(download_path).await?;

    return Ok(UniversalDownloadResult {
        description: "Successfully downloaded mod file",
        status: DownloadStatus::Downloaded,
    });
}

fn check_availability(sides: &Vec<ModTargetSide>, filename: &str) -> bool {
    let mut paths = vec![
        env::current_dir().unwrap().join("mods").join("downloading").join(filename),
    ];

    if sides.contains(&ModTargetSide::Client) {
        paths.push(env::current_dir().unwrap().join("mods").join("client").join(filename));
    }

    if sides.contains(&ModTargetSide::Server) {
        paths.push(env::current_dir().unwrap().join("mods").join("server").join(filename));
    }

    return paths.iter().any(|path| path.exists());
}
