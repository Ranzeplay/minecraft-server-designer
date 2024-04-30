use crate::models::config::{AppConfig, ModMetadata, ModProvider};
use crate::models::startup_args::AddModCommand;
use crate::providers::curseforge_provider::CurseForgeProvider;
use crate::providers::modrinth_provider::ModrinthProvider;

pub async fn add_mod(mod_to_add: AddModCommand) -> anyhow::Result<()> {
    let mut config = AppConfig::load();

    let provider = mod_to_add.provider.clone();
    
    let mut metadata = ModMetadata {
        name: mod_to_add.name.unwrap_or(String::new()),
        mod_id: mod_to_add.id,
        provider: mod_to_add.provider,
        version: mod_to_add.version.unwrap_or(String::new()),
        categories: vec![],
        sides: mod_to_add.side.unwrap_or(vec![]),
    };
    
    match provider {
        ModProvider::Modrinth => {
            let fetched_data = ModrinthProvider::get_mod_metadata(metadata.clone(), &config)
                .await
                .expect("Failed to fetch mod metadata");
            
            if metadata.sides.is_empty() {
                metadata.sides = fetched_data.sides;
            }
            if metadata.name.is_empty() {
                metadata.name = fetched_data.name;
            }
            if metadata.version.is_empty() {
                metadata.version = fetched_data.version;
            }
        }
        ModProvider::CurseForge => {
            let fetched_data = CurseForgeProvider::get_mod_metadata(metadata.clone(), &config)
                .await
                .expect("Failed to fetch mod metadata");

            if metadata.sides.is_empty() {
                metadata.sides = fetched_data.sides;
            }
            if metadata.name.is_empty() {
                metadata.name = fetched_data.name;
            }
            if metadata.version.is_empty() {
                metadata.version = fetched_data.version;
            }
        }
    }
    
    config.mods.push(metadata);

    config.save().expect("Failed to save configurations");
    
    Ok(())
}
