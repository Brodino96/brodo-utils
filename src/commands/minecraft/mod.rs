pub mod order;

use clap::{Args, Subcommand};

#[derive(Args)]
pub struct MinecraftArgs {
    #[command(subcommand)]
    pub command: MinecraftCommands,
}

#[derive(Subcommand)]
pub enum MinecraftCommands {
    /// Sort a Minecraft log file in chronological order
    Order(order::OrderArgs),
}

pub fn run(args: MinecraftArgs) {
    match args.command {
        MinecraftCommands::Order(a) => order::run(a),
    }
}
