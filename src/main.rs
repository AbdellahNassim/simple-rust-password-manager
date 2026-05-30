pub mod models;
pub mod services;
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
            add_credential(service, username);
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
