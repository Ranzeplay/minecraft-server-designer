use std::{env, fs};
use std::fs::File;
use std::io::Write;
use crate::viewmodel::config::{AppConfig, ModMetadata};
use crate::viewmodel::startup_args::AddModCommand;

pub fn add_mod(mod_to_add: AddModCommand) {
    let config_path = &env::current_dir().unwrap().join("config.yml");
    let config_content = fs::read_to_string(config_path)
        .expect("Failed to read config file");

    let mut config = serde_yaml::from_str::<AppConfig>(&*config_content).unwrap();

    println!("Adding mod {}", mod_to_add.name);
    let new_mod = ModMetadata{
        name: mod_to_add.name,
        mod_id: mod_to_add.id,
        provider: mod_to_add.provider,
        version: mod_to_add.version,
        categories: vec![],
        sides: mod_to_add.side,
    };
    config.mods.push(new_mod);

    fs::remove_file(config_path).unwrap();
    let mut file = File::create(config_path).unwrap();
    file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes()).unwrap();
}
