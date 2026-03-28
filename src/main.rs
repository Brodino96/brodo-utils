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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hasher(args) => commands::hasher::run(args),
    }
}
