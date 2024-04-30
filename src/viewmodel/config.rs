use std::{env, fs};
use std::fs::File;
use std::io::Write;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AppConfig {
    pub game_version: String,
    pub mod_loader: ModLoader,
    pub mods: Vec<ModMetadata>,
    pub curse_api_key: String,
}

impl AppConfig {
    pub fn load() -> AppConfig {
        let config_path = &env::current_dir().unwrap().join("config.yml");
        let config_content = fs::read_to_string(config_path)
            .expect("Failed to read config file");

        return serde_yaml::from_str::<AppConfig>(&*config_content)
            .expect("Failed to parse config file");
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let content = serde_yaml::to_string(&self)?;
        let path = env::current_dir().unwrap().join("config.yml");
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ValueEnum)]
pub enum ModLoader {
    Vanilla,
    Fabric,
    Spigot,
    Forge,
    NeoForge
}

impl ModLoader {
    pub fn to_string(&self) -> &str {
        return match &self {
            ModLoader::Vanilla => "vanilla",
            ModLoader::Fabric => "fabric",
            ModLoader::Spigot => "spigot",
            ModLoader::Forge => "forge",
            ModLoader::NeoForge => "neoforge"
        }
    }
    
    pub fn to_curseforge_id(&self) -> i32 {
        return match &self {
            ModLoader::Fabric => 4,
            ModLoader::Forge => 1,
            ModLoader::NeoForge => 6,
            _ => 0
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ModMetadata {
    pub name: String,
    pub mod_id: String,
    pub provider: ModProvider,
    pub version: String,
    pub categories: Vec<String>,
    pub sides: Vec<ModTargetSide>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ValueEnum)]
pub enum ModProvider {
    Modrinth,
    CurseForge
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ValueEnum)]
pub enum ModTargetSide {
    Client,
    Server
}
