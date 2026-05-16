pub mod death_counter;
pub mod kill_counter;
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
    /// Count how many times each player was killed
    DeathCounter(death_counter::DeathCounterArgs),
    /// Count player kills from a log file
    KillCounter(kill_counter::KillCounterArgs),
}

pub fn run(args: MinecraftArgs) {
    match args.command {
        MinecraftCommands::DeathCounter(a) => death_counter::run(a),
        MinecraftCommands::Order(a) => order::run(a),
        MinecraftCommands::KillCounter(a) => kill_counter::run(a),
    }
}
