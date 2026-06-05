pub mod models;
pub mod services;
pub mod data;
pub mod errors;
pub mod crypto;
use clap::{Parser, Subcommand};
use data::database::{create_pool,initialize_database};
use services::commands::{add_credential, get_credential, delete_credential, list_credentials};
use data::sqlite_repository::SqliteCredentialRepository;
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

#[tokio::main]
async fn main() {
    let pool = create_pool().await.expect("Failed to create database pool");
    initialize_database(&pool).await.expect("Failed to initialize database");
    let mut repository = SqliteCredentialRepository::new(pool.clone());
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { service, username } => {
            if let Err(e) = add_credential(&mut repository, service, username).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Get { service } => {
            if let Err(e) = get_credential(&repository, service).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::List => {
            if let Err(e) = list_credentials(&repository).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Delete { service } => {
            if let Err(e) = delete_credential(&mut repository, service).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}
