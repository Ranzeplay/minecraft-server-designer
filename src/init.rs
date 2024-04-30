use std::{env, fs};
use crate::viewmodel::config::{AppConfig, ModLoader};

use anyhow::Result;

pub fn init_directory_structure() {
    let dir = env::current_dir()
        .expect("Failed to get current directory");

    println!("Current directory: {:?}", dir);

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

fn init_config(game_version: String, mod_loader: ModLoader) -> Result<()> {
    let config = AppConfig {
        game_version,
        mod_loader,
        mods: vec![],
        curse_api_key: "".to_string(),
    };

    config.save()
}

pub fn init_all(game_version: String, mod_loader: ModLoader) {
    init_directory_structure();
    init_config(game_version, mod_loader).expect("Failed to create config file");
}