use clap::{Args, Parser, Subcommand};
use crate::viewmodel::config::ModLoader;

#[derive(Parser, Debug)]
#[command(author = "Jeb Feng", version = "0.1", about = "A designer for Minecraft server", long_about = None)]
pub struct StartupArgs {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Initialize current directory as a MSD root")]
    Init(InitCommand)
}

#[derive(Args, Debug)]
pub struct InitCommand {
    #[arg(short, long, default_value = "latest")]
    pub game_version: String,
    #[arg(short, long, default_value = "vanilla")]
    pub server_mod_loader: ModLoader,
    #[arg(short, long, default_value = "vanilla")]
    pub client_mod_loader: ModLoader,
}
