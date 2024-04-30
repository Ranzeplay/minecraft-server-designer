use crate::viewmodel::config::{ModLoader, ModTargetSide};

pub struct DownloadModMetadata {
    pub name: String,
    pub mod_id: String,
    pub mod_version: String,
    pub mod_loader: ModLoader,
    pub game_version: String,
    pub sides: Vec<ModTargetSide>,
}
