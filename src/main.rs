use clap::{Parser, Subcommand};
use password_manager::data::database::{create_pool,run_migrations};
use password_manager::services::commands::{add_credential, get_credential, delete_credential, list_credentials,setup_vault};
use password_manager::data::sqlite_repository::SqliteCredentialRepository;

use password_manager::auth::auth::authenticate;
use password_manager::services::password_generator::{generate_password, validate_length};
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
    Generate { 
        length: usize,
        #[arg(long)]
        no_symbols: bool,
        #[arg(long)]
        copy: bool,
     },
}

#[tokio::main]
async fn main() {
    let pool = create_pool("sqlite:vault.db").await.expect("Failed to create database pool");
    run_migrations(&pool).await.expect("Failed to run migrations");
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
        Commands::Generate { length, no_symbols, copy } => {
           if let Err(e) = validate_length(length) {
            eprintln!("Error: {}", e);
            return;
           }
           let password = generate_password(length, no_symbols);
           println!("Generated password: {}", password);
           if copy {
            arboard::Clipboard::new().unwrap().set_text(password).unwrap();
            println!("Password copied to clipboard");
           }
        }
    }
}
