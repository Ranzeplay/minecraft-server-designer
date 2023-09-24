use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AppConfig {
    pub game_version: String,
    pub mod_loader: ModLoader,
    pub mods: Vec<ModMetadata>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, ValueEnum)]
pub enum ModLoader {
    Vanilla,
    Fabric,
    Spigot,
    Forge
}

impl ModLoader {
    pub fn to_string(&self) -> &str {
        return match &self {
            ModLoader::Vanilla => "vanilla",
            ModLoader::Fabric => "fabric",
            ModLoader::Spigot => "spigot",
            ModLoader::Forge => "forge",
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
