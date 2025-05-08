use prettytable::{row, Table};
use crate::models::config::{AppConfig, ModMetadata, ModProvider, ModTargetSide};
use crate::models::startup_args::AddModCommand;
use crate::providers::curseforge_provider::CurseForgeProvider;
use crate::providers::local_provider::LocalProvider;
use crate::providers::modrinth_provider::ModrinthProvider;
use colored::Colorize;
use tokio::task;

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

pub fn list_mods(side_filter: Option<ModTargetSide>) {
    let mods = AppConfig::load().mods;

    let mut table = Table::new();
    table.add_row(row![b->"Mod ID", b->"Name", b->"Provider", b->"Version", b->"Sides"]);
    
    let filtered_mods: Vec<&ModMetadata> = if let Some(side) = side_filter {
        mods.iter().filter(|mod_meta| mod_meta.sides.contains(&side)).collect()
    } else {
        mods.iter().collect()
    };

    for mod_meta in filtered_mods {
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

pub async fn check_config(game_version: Option<String>) {
    let mut config = AppConfig::load();
    if game_version.is_some() {
        config.game_version = game_version.unwrap();
    }

    let mut handles = vec![];
    for mc_mod in config.clone().mods {
        let config_cloned = config.clone();
        handles.push(task::spawn(async move {
            return match mc_mod.provider {
                ModProvider::Modrinth => ModrinthProvider::get_mod_metadata(mc_mod.clone(), &config_cloned).await,
                ModProvider::CurseForge => CurseForgeProvider::get_mod_metadata(mc_mod.clone(), &config_cloned).await,
                _ => Ok(mc_mod.clone())
            }
        }));
    }

    let mut results = vec![];
    let mut failed_mods = vec![];
    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.await;
        match result {
            Ok(Ok(_)) => results.push(Ok(())),
            Ok(Err(e)) => {
                results.push(Err(()));
                failed_mods.push((config.mods[i].clone(), e.to_string()));
            },
            Err(e) => {
                results.push(Err(()));
                failed_mods.push((config.mods[i].clone(), e.to_string()));
            }
        }
    }

    println!("{}", "Summary".bold());
    println!("Total: {} | Success: {} | Fail: {}",
             results.len(),
             results.iter().filter(|r| r.is_ok()).count(),
             results.iter().filter(|r| r.is_err()).count());

    if !failed_mods.is_empty() {
        println!("\n{}", "Failed Mods:".bold().red());
        for (mod_meta, error) in failed_mods {
            println!("{} ({}): {}", mod_meta.name.bold(), mod_meta.provider, error);
        }
    }
}

