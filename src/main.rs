pub mod models;
pub mod services;
pub mod data;
pub mod errors;
use clap::{Parser, Subcommand};

use services::commands::{add_credential, get_credential, delete_credential, list_credentials};
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { service: String, username: String},
    Get {
        service: String,
    },
    List,
    Delete { service: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { service, username } => {
            if let Err(e) = add_credential(service, username) {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Get { service } => {
            get_credential(service);
        },
        Commands::List => {
            list_credentials();
        },
        Commands::Delete { service } => {
            delete_credential(service);
        },
        _ => {
            println!("Invalid command");
        },
    }
}
