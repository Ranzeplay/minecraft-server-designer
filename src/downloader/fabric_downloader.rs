use std::env;
use std::process::Stdio;
use colored::Colorize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use crate::downloader::vanilla_downloader::download_vanilla_server;

pub async fn download_fabric_server(game_version: String) -> anyhow::Result<()> {
    println!("{}", "Downloading Fabric server".bold());
    let fabric_metadata_response = reqwest::get("https://maven.fabricmc.net/net/fabricmc/fabric-installer/maven-metadata.xml")
        .await?
        .text()
        .await?;

    let json_response = xmltojson::to_json(fabric_metadata_response.as_str())
        .expect("Failed to parse response to JSON");

    let latest_installer_version = json_response["metadata"]["versioning"]["release"]
        .as_str()
        .expect("Failed to get latest fabric installer version");

    let latest_installer_version_text = format!("Using latest Fabric installer with version {}", latest_installer_version);
    println!("{}", latest_installer_version_text.bright_blue());

    let file_content = reqwest::get(format!("https://maven.fabricmc.net/net/fabricmc/fabric-installer/{0}/fabric-installer-{0}.jar", latest_installer_version))
        .await?
        .bytes()
        .await?;
    let server_path = &env::current_dir().unwrap().join("server");
    let installer_path =  server_path.join("fabric-installer.jar");
    let mut file = File::create(installer_path).await?;
    file.write_all(&file_content).await?;

    // Execute commands to download via Fabric Installer
    println!("{}", "Downloading Fabric server via Fabric Installer".bright_blue());
    let mut proc = Command::new("java")
        .current_dir(server_path)
        .args(&["-jar", "fabric-installer.jar", "server", "-mcversion", &*game_version])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    proc.wait().await?;

    download_vanilla_server(game_version).await?;

    Ok(())
}
