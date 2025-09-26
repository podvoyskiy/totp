use std::{path::PathBuf, process::Command};

use rpassword::read_password;

use crate::prelude::{AppError, Totp};

pub struct Crypto;

impl Crypto {
    pub fn encrypting(service_name: &str) -> Result<(), AppError> {
        //  echo "your_base32_secret_here" | gpg -c > ~/.config/totp/your_service.gpg
        println!("TODO {service_name} Encrypting...");
        Ok(())
    }

    pub fn decrypting(service: &PathBuf) -> Result<(), AppError> {
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
}