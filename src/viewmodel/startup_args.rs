use clap::{Args, Parser, Subcommand};
use crate::viewmodel::config::{ModLoader, ModProvider, ModTargetSide};

#[derive(Parser, Debug)]
#[command(author = "Jeb Feng", version = "0.1", about = "A designer for Minecraft server", long_about = None)]
pub struct StartupArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Initialize current directory as a MSD root")]
    Init(InitCommand),
    #[command(subcommand)]
    #[clap(about = "Manage mods used in the server and client")]
    Mod(ModCommand),
    #[clap(about = "Build current configuration")]
    Build
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
    Add(AddModCommand)
}

#[derive(Args, Debug)]
pub struct AddModCommand {
    #[arg(short, long, required = true, help = "The name of the mod")]
    pub name: String,
    #[arg(short, long, required = true, help = "The mod id bind with download source")]
    pub id: String,
    #[arg(short, long, required = true, help = "Version of the mod")]
    pub version: String,
    #[arg(short, long, required = true, help = "Mod loader for the mod")]
    pub loader: ModLoader,
    #[arg(short, long, required = true, help = "The provider of the mod (Support CurseForge and Modrinth)")]
    pub provider: ModProvider,
    #[arg(short, long, required = true, help = "Which side should the mod be installed on")]
    pub side: Vec<ModTargetSide>
}
