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
    pub use crate::crypto::{Crypto, NativeCrypto, create_crypto};
    #[allow(unused_imports)]
    pub use crate::crypto::GpgCrypto;
    pub use crate::colorize::Colorize;
}
use prelude::*;

use std::{env, io};

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    let crypto = create_crypto();
    
    let storage = Storage::new(crypto)?;

    match args.len() {
        1 => {
            if storage.services.is_empty() {
                println!("{}", "No services found. Run 'totp --add' to add first service".warning());
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

            storage.crypto.decrypting(&storage.services[choice - 1])
        }
        _ => {
            if &args[1] == "--add" {
                println!("Enter service name:");
                let mut service_name = String::new();
                io::stdin().read_line(&mut service_name)?;
                service_name = service_name.trim().to_string();

                if !Storage::validate_file_name(&service_name) {
                    return Err(AppError::InvalidInput("incorrect service name".into()));
                }

                storage.crypto.encrypting(&storage.get_service_path(&service_name))?;
                return Ok(());
            }

            print_help();
            Ok(())
        }
    }
}

fn print_help() {
    println!("{}{}", "Usage:".warning().bold(), " totp".info().bold());
    println!("{}", "Options:".warning().bold());
    println!("{}    Add new service", "  --add".info().bold());
}
