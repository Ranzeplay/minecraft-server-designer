use crate::models::config::{AppConfig, ModMetadata};
use crate::models::startup_args::AddModCommand;

pub fn add_mod(mod_to_add: AddModCommand) {
    let mut config = AppConfig::load();

    println!("Adding mod {}", mod_to_add.name);
    let new_mod = ModMetadata {
        name: mod_to_add.name,
        mod_id: mod_to_add.id,
        provider: mod_to_add.provider,
        version: mod_to_add.version,
        categories: vec![],
        sides: mod_to_add.side,
    };
    config.mods.push(new_mod);

    config.save().expect("Failed to save configurations");
}
