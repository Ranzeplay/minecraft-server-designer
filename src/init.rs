use std::{env, fs};
use std::fs::File;
use std::io::Write;
use crate::viewmodel::config::{AppConfig, ModLoader};

use anyhow::Result;

fn init_directory_structure() {
    let dir = env::current_dir()
        .expect("Failed to get current directory");

    // Create server directory
    fs::create_dir_all(dir.clone().join("server"))
        .expect("Failed to create server directory");

    // Create mods directory
    fs::create_dir_all(dir.clone().join("mods"))
        .expect("Failed to create server directory");

    // Create mods temp directory
    fs::create_dir_all(dir.clone().join("mods").join("downloading"))
        .expect("Failed to create server directory");

    // Create mods directory for target sides
    fs::create_dir_all(dir.clone().join("mods").join("server"))
        .expect("Failed to create server directory");
    fs::create_dir_all(dir.clone().join("mods").join("client"))
        .expect("Failed to create server directory");

    // Create client directory
    fs::create_dir_all(dir.clone().join("client"))
        .expect("Failed to create server directory");

    println!("Successfully initialized directories")
}

fn init_config(game_version: String, server_mod_loader: ModLoader, client_mod_loader: ModLoader) -> Result<()> {
    let config = AppConfig {
        game_version,
        client_mod_loader,
        server_mod_loader,
        mods: vec![],
    };

    let content = serde_yaml::to_string(&config)?;
    let path = env::current_dir().unwrap().join("config.yml");
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn init_all(game_version: String, server_mod_loader: ModLoader, client_mod_loader: ModLoader) {
    init_directory_structure();
    init_config(game_version, server_mod_loader, client_mod_loader).expect("Failed to create config file");
}