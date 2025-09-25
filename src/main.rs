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

use std::{env, path::PathBuf, process::Command};
use rpassword::read_password;

fn main() -> Result<(), AppError> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0); //remove program name

    let storage = Storage::load()?;
    //TODO if error - show "No services found. Run 'totp --add' to add first service"

    match args.len() {
        0 => {
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
        1 => {
            if &args[0] == "--add" {
                println!("TODO Add service");
                return Ok(());
            } 

            if let Some(service) = storage.find_service_by_name(&args[0]) {
                decrypting(service)
            } else {
                println!("service {} not found in config", &args[0]);
                Ok(())
            }
        }
        _ => {
            println!("Show help");
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
