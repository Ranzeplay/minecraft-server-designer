use prettytable::{row, Table};
use crate::models::config::{AppConfig, ModMetadata, ModProvider};
use crate::models::startup_args::AddModCommand;
use crate::providers::curseforge_provider::CurseForgeProvider;
use crate::providers::local_provider::LocalProvider;
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
        ModProvider::Local => {
            let data = LocalProvider::get_mod_metadata(metadata.clone(), &config)
                .await
                .expect("Failed to fetch mod metadata");
            
            metadata.mod_id = data.mod_id;
            metadata.sides = data.sides;
        }
    }
    
    config.mods.push(metadata);

    config.save().expect("Failed to save configurations");
    
    println!("Mod added successfully");
    
    Ok(())
}

pub fn list_mods() {
    let mods = AppConfig::load().mods;

    let mut table = Table::new();
    table.add_row(row![b->"Mod ID", b->"Name", b->"Provider", b->"Version", b->"Sides"]);
    for mod_meta in mods {
        let side_string = mod_meta.sides.iter().map(|side| side.to_string()).collect::<Vec<&str>>().join(" ");
        table.add_row(row![mod_meta.mod_id, mod_meta.name, mod_meta.provider, mod_meta.version, side_string]);
    }
    
    table.printstd();
    println!("Showing {} mods", table.len());
}

pub fn remove_mod(id: String) {
    let mut config = AppConfig::load();
    
    if config.mods.iter().find(|mod_meta| mod_meta.mod_id == id).is_none() {
        println!("Mod not found");
        return;
    }
    
    config.mods.retain(|mod_meta| mod_meta.mod_id != id);
    config.save().expect("Failed to save configurations");
    
    println!("Mod removed successfully");
}
