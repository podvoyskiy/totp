#![warn(clippy::all)]

mod totp;
mod errors;
mod storage;
mod crypto;
mod colorize;
mod prelude { 
    pub use crate::totp::Totp;
    pub use crate::errors::AppError;
    pub use crate::storage::Storage;
    pub use crate::crypto::{Crypto, NativeCrypto, GpgCrypto, create_crypto};
    pub use crate::colorize::Colorize;
}
use prelude::*;

use std::{env, io, process};
use rpassword::read_password;

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    let crypto = create_crypto();
    
    let storage = Storage::new(crypto)?;

    match args.len() {
        1 => {
            let service_index = select_service(&storage)?;
            let secret = storage.crypto.decrypting(&storage.services[service_index])?;
            Totp::display(&secret)?;
            Ok(())
        }
        2 => handle_command(&storage, &args[1]),
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn handle_command(storage: &Storage, command: &str) -> Result<(), AppError> {
    match command {
        "--add" => add_service(storage),
        "--del" => delete_service(storage),
        "--export" => export_services(storage),
        "--import" => import_services(storage),
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn select_service(storage: &Storage)-> Result<usize, AppError> {
    if storage.services.is_empty() {
        println!("{}", "No services found. Run 'totp --add' to add first service".warning());
        process::exit(0);
    }
                
    println!("Select service:");
    storage.services.iter().enumerate().for_each(|(index, path)| {
        if let Some(item) = path.file_stem() {
            println!("{} : {}", index + 1, item.to_string_lossy());
        }
    });

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse()?;

    if choice == 0 || choice > storage.services.len() {
        return Err(AppError::InvalidInput("Invalid service selection".into()));
    }
    Ok(choice - 1)
}

fn add_service(storage: &Storage) -> Result<(), AppError> {
    println!("Enter service name:");
    let mut service_name = String::new();
    io::stdin().read_line(&mut service_name)?;
    service_name = service_name.trim().to_string();

    if !Storage::validate_file_name(&service_name) {
        return Err(AppError::InvalidInput("incorrect service name".into()));
    }

    println!("Insert TOTP secret:");
    let secret = read_password()?;

    storage.crypto.encrypting(&storage.get_service_path(&service_name), secret)?;
    Ok(())
}

fn delete_service(storage: &Storage) -> Result<(), AppError> {
    let service_index = select_service(storage)?;
    storage.delete_service(service_index)
}

fn export_services(storage: &Storage) -> Result<(), AppError> {
    if storage.services.is_empty() {
        println!("{}", "No services found for export".warning());
        process::exit(0);
    }
    storage.export_services()
}

fn import_services(storage: &Storage) -> Result<(), AppError> {
    storage.import_services()
}

fn print_help() {
    println!("{}{}", "Usage:".warning().bold(), " totp".info());
    println!("{}", "Options:".warning().bold());
    println!("{}       Add new service", "  --add".info());
    println!("{}       Delete service", "  --del".info());
    println!("{}    Export services to json", "  --export".info());
    println!("{}    Import services from json", "  --import".info());
}
