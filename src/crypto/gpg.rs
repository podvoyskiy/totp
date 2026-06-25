use std::{cell::RefCell, io::Write, path::Path, process::{Command, Stdio}};

use crate::prelude::*;

pub struct GpgCrypto {
    password_cache: RefCell<Option<String>>
}

impl GpgCrypto {
    pub fn default() -> Self {
        Self { password_cache: RefCell::new(None) }
    } 

    pub fn is_available() -> bool {
        Command::new("gpg")
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
    }
}

impl Crypto for GpgCrypto {
    fn get_extension_files(&self) -> &'static str {
        "gpg"
    }

    fn encrypting(&self, path_to_file: &Path, secret: String) -> Result<()> {
        self.validate_secret(&secret)?;
        let password = self.get_password()?;

        println!("{}", "Encrypting...".cyan());

        let input = format!("{password}\n{secret}\n");

        let mut cmd = Command::new("gpg")
            .arg("--batch") //no interactive prompts
            .arg("--yes") //overwrite output files
            .arg("--passphrase-fd") //read passphrase from file descriptor
            .arg("0") //use stdin as passphrase source
            .arg("-c") //symmetric encryption
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

        println!("{}", format!("Successfully encrypted and saved to: {}", path_to_file.display()).green());

        Ok(())
    }

    fn decrypting(&self, path_to_file: &Path) -> Result<String> {
        let password = self.get_password()?;

        println!("{}", "Decrypting...".cyan());

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
        Ok(secret)
    }
    
    fn get_password_cache(&self) -> &RefCell<Option<String>> {
        &self.password_cache
    }
}