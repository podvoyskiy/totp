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

use std::env;

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
                storage.print_help();
                return Ok(());
            } 

            if &args[1] == "--add" {
                println!("Enter the service name:");
                let mut new_service = String::new();
                std::io::stdin().read_line(&mut new_service)?;
                //TODO validate
                Crypto::encrypting(new_service.trim())?;
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
            storage.print_help();
            Ok(())
        }
    }
}
