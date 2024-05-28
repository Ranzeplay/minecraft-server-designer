use clap::{Args, Parser, Subcommand};
use crate::models::config::{ModLoader, ModProvider, ModTargetSide};

#[derive(Parser, Debug)]
#[command(author = "Jeb Feng", version = "0.1", about = "A designer for Minecraft server", long_about = None)]
pub struct StartupArgs {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short ='c', long = "config", help = "The configuration file path", default_value = "./config.yml")]
    pub config_path: String
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Initialize current directory as a MSD root")]
    Init(InitCommand),
    #[command(subcommand)]
    #[clap(about = "Manage mods used in the server and client")]
    Mod(ModCommand),
    #[clap(about = "Build current configuration")]
    Build(BuildCommand),
}

#[derive(Args, Debug)]
pub struct InitCommand {
    #[arg(short, long, default_value = "latest")]
    pub game_version: String,
    #[arg(short = 'l', long, default_value = "vanilla")]
    pub mod_loader: ModLoader,
}

#[derive(Subcommand, Debug)]
pub enum ModCommand {
    #[clap(about = "Add mod to current configuration")]
    Add(AddModCommand),
    #[clap(about = "List all added mods")]
    List,
    #[clap(about = "Remove a mod from the added mod")]
    Remove(RemoveModCommand),
    #[clap(about = "Check for mod metadata")]
    Check(CheckModCommand),
}

#[derive(Args, Debug)]
pub struct AddModCommand {
    #[arg(short, long, required = false, help = "The name of the mod")]
    pub name: Option<String>,
    #[arg(short, long, required = true, help = "The mod id bind with download source")]
    pub id: String,
    #[arg(short, long, required = false, help = "Version of the mod")]
    pub version: Option<String>,
    #[arg(short, long, required = true, help = "The provider of the mod (Support CurseForge and Modrinth)")]
    pub provider: ModProvider,
    #[arg(short, long, required = false, help = "Which side should the mod be installed on")]
    pub side: Option<Vec<ModTargetSide>>
}

#[derive(Args, Debug)]
pub struct RemoveModCommand {
    #[arg(short, long, required = true, help = "The mod id bind with download source")]
    pub id: String,
}

#[derive(Args, Debug)]
pub struct BuildCommand {
    #[arg(short, long, help = "Skip re-downloading server files even if they don't exist", default_value_t = false)]
    pub skip_server: bool,
    #[arg(short, long, help = "Skip re-downloading mod files if they already exist", default_value_t = false)]
    pub force_download: bool
}

#[derive(Args, Debug)]
pub struct CheckModCommand {
    #[arg(short = 'v', long, help = "Check on specific game version")]
    pub game_version: Option<String>,
}
