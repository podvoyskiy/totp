#![warn(clippy::all)]

mod totp;
mod errors;
mod storage;
mod prelude { 
    pub use crate::totp::Totp;
    pub use crate::errors::AppError;
    pub use crate::storage::Storage;
}
use prelude::*;

use colored::*;

use std::{env, path::PathBuf, process::Command};
use rpassword::read_password;

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

            decrypting( &storage.services[choice - 1])
        }
        2 => {
            if &args[1] == "--help" {
                storage.print_help();
                return Ok(());
            } 

            if &args[1] == "--add" {
                println!("TODO Add service");
                return Ok(());
            } 

            if let Some(service) = storage.find_service_by_name(&args[1]) {
                decrypting(service)
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

fn decrypting(service: &PathBuf) -> Result<(), AppError> {
    println!("Enter password:");
    let password = read_password()?;

    println!("Decrypting...");

    let output = Command::new("gpg")
        .arg("-d")
        .arg("-q")
        .arg("--batch")
        .arg("--passphrase")
        .arg(password) 
        .arg(service)
        .output()?;
    
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::InvalidInput(error_msg.trim().into()));
    }

    let secret = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    Totp::display(&secret)?;

    Ok(())
}
