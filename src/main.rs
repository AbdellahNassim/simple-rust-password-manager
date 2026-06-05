pub mod models;
pub mod services;
pub mod data;
pub mod errors;
pub mod crypto;
pub mod master_password;
pub mod auth;
use clap::{Parser, Subcommand};
use data::database::{create_pool,initialize_database};
use services::commands::{add_credential, get_credential, delete_credential, list_credentials,setup_vault};
use data::sqlite_repository::SqliteCredentialRepository;

use crate::auth::auth::authenticate;
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
    Setup,
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await.expect("Failed to create database pool");
    initialize_database(&pool).await.expect("Failed to initialize database");
    let mut repository = SqliteCredentialRepository::new(pool.clone());
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { service, username } => {
            let crypto_service = authenticate(&mut repository).await.expect("Failed to authenticate");
            if let Err(e) = add_credential(&mut repository, service, username, &crypto_service).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Get { service } => {
            let crypto_service = authenticate(&mut repository).await.expect("Failed to authenticate");
            if let Err(e) = get_credential(&repository, service, &crypto_service).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::List => {
            let _crypto_service = authenticate(&mut repository).await.expect("Failed to authenticate");
            if let Err(e) = list_credentials(&repository).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Delete { service } => {
            let _crypto_service = authenticate(&mut repository).await.expect("Failed to authenticate");
            if let Err(e) = delete_credential(&mut repository, service).await {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Setup => {
            if let Err(e) = setup_vault(&mut repository).await {
                eprintln!("Error: {}", e);
            }
        }
    }
}
