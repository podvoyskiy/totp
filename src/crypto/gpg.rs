use std::{io::Write, path::Path, process::{Command, Stdio}};

use crate::{prelude::{AppError, Totp, Crypto, Colorize}};

pub struct GpgCrypto;

impl GpgCrypto {
    pub fn is_available() -> bool {
        Command::new("gpg")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

impl Crypto for GpgCrypto {
    fn get_extension_files(&self) -> &str {
        "gpg"
    }

    fn encrypting(&self, path_to_file: &Path) -> Result<(), AppError> {
        let secret = self.get_secret()?;
        let password = self.get_password()?;

        println!("{}", "Encrypting...".info());

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
            return Err(AppError::Encrypt(format!("Encryption failed: {}", error_msg.trim())));
        }

        println!("{}", format!("Successfully encrypted and saved to: {}", path_to_file.display()).success());

        Ok(())
    }

    fn decrypting(&self, path_to_file: &Path) -> Result<(), AppError> {
        let password = self.get_password()?;

        println!("{}", "Decrypting...".info());

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
            return Err(AppError::Encrypt(format!("Decryption failed: {}", error_msg.trim())));
        }

        let secret = String::from_utf8_lossy(&output.stdout).trim().to_owned();

        Totp::display(&secret)?;

        Ok(())
    }
}