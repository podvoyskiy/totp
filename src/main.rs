#![warn(clippy::all)]

mod totp;
mod errors;
mod storage;
mod crypto;
mod prelude { 
    pub use crate::totp::Totp;
    pub use crate::errors::AppError;
    pub use crate::storage::Storage;
    pub use crate::crypto::Crypto;
}
use prelude::*;

use colored::*;

use std::{env, io};

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    let storage = Storage::load()?;

    match args.len() {
        1 => {
            if storage.services.is_empty() {
                println!("{}", "No services found. Run 'totp --add' to add first service".yellow());
                return Ok(());
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

            Crypto::decrypting( &storage.services[choice - 1])
        }
        2 => {
            if &args[1] == "--help" {
                print_help();
                return Ok(());
            } 

            if &args[1] == "--add" {
                println!("Enter service name:");
                let mut service_name = String::new();
                io::stdin().read_line(&mut service_name)?;
                service_name = service_name.trim().to_string();

                if !Storage::validate_file_name(&service_name) {
                    return Err(AppError::InvalidInput("incorrect service name".into()));
                }

                Crypto::encrypting(&storage, &service_name)?;
                return Ok(());
            } 

            if let Some(service) = storage.find_service_by_name(&args[1]) {
                Crypto::decrypting(service)
            } else {
                println!("{}", format!("service {} not found in config", &args[1]).yellow());
                Ok(())
            }
        }
        _ => {
            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("{}{}{}", "Usage:".yellow().bold(), " totp".blue().bold(), " [OPTION]".blue());
    println!("{}", "Options:".yellow().bold());
    println!("{}               Show this help", "  --help".blue().bold());
    println!("{}                Add new service", "  --add".blue().bold());
    println!("{}       Service selection at start", "  {service_name}".blue().bold());
}
