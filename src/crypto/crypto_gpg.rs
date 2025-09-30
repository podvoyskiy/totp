use std::{io::Write, path::PathBuf, process::{Command, Stdio}};

use colored::Colorize;
use rpassword::read_password;

use crate::{prelude::{AppError, Totp, Crypto}};

pub struct CryptoGpg;

impl Crypto for CryptoGpg {
    fn encrypting(path_to_file: &PathBuf) -> Result<(), AppError> {
        println!("Insert TOTP secret:");
        let secret = read_password()?;

        Totp::generate(&secret)?; //just to validate secret

        println!("Enter password for encryption:");
        let password = read_password()?;

        println!("{}", "Encrypting...".blue());

        let input = format!("{password}\n{secret}\n");

        let mut cmd = Command::new("gpg")
            .arg("--batch")
            .arg("--passphrase-fd")
            .arg("0")
            .arg("-c")
            .arg("-o")
            .arg(path_to_file)
            .stdin(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = cmd.stdin.take() {
            stdin.write_all(input.as_bytes())?;
        }

        let output = cmd.wait_with_output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::InvalidInput(format!("Encryption failed: {}", error_msg.trim())));
        }

        println!("{}", format!("Successfully encrypted and saved to: {}", path_to_file.display()).green());

        Ok(())
    }

    fn decrypting(path_to_file: &PathBuf) -> Result<(), AppError> {
        println!("Enter password for decryption:");
        let password = read_password()?;

        println!("{}", "Decrypting...".blue());

        let output = Command::new("gpg")
            .arg("-d")
            .arg("-q")
            .arg("--batch")
            .arg("--passphrase")
            .arg(password) 
            .arg(path_to_file)
            .output()?;
        
        
        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::InvalidInput(format!("Decryption failed: {}", error_msg.trim())));
        }

        let secret = String::from_utf8_lossy(&output.stdout).trim().to_owned();

        Totp::display(&secret)?;

        Ok(())
    }
}