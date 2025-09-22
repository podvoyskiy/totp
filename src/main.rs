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

use std::{process::Command};
use rpassword::read_password;

fn main() -> Result<(), AppError> {
    let storage = Storage::load()?;

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

    println!("Enter password:");
    let password = read_password()?;

    println!("Decrypting...");

    let output = Command::new("gpg")
        .arg("-d")
        .arg("-q")
        .arg("--batch")
        .arg("--passphrase")
        .arg(password) 
        .arg(&storage.services[choice - 1])
        .output()?;
    
    
    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(AppError::InvalidInput(error_msg.trim().into()));
    }

    let secret = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    Totp::display(&secret)?;

    Ok(())
}
