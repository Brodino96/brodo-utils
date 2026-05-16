use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "brodo-utils")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hasher(commands::hasher::HasherArgs),
    /// Minecraft-related utilities
    Minecraft(commands::minecraft::MinecraftArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hasher(args) => commands::hasher::run(args),
        Commands::Minecraft(args) => commands::minecraft::run(args),
    }
}
