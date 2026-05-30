use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,
    Get,
    List,
    Remove,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add => {
            println!("Adding a new password");
        },
        Commands::Get => {
            println!("Getting a password");
        },
        Commands::List => {
            println!("Listing all passwords");
        },
        Commands::Remove => {
            println!("Removing a password");
        },
        _ => {
            println!("Invalid command");
        },
    }
}
