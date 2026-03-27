use std::{cell::RefCell, path::Path};

use rpassword::read_password;

use crate::prelude::{AppError, Totp, NativeCrypto, GpgCrypto, Colorize};

pub trait Crypto {
    fn get_extension_files(&self) -> &str;

    fn encrypting(&self, path_to_file: &Path, secret: String) -> Result<(), AppError>;

    fn decrypting(&self, path_to_file: &Path) -> Result<String, AppError>;

    fn get_password_cache(&self) -> &RefCell<Option<String>>;

    fn validate_secret(&self, secret: &str) -> Result<(), AppError> {
        Totp::generate(secret).map(|_| ())?;
        Ok(())
    }

    fn get_password(&self) -> Result<String, AppError> {
        let password_cache = self.get_password_cache();

        if let Some(stored) = password_cache.borrow().clone() {
            println!("{}", "Use the same password? (y/n):".success());
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.trim().to_lowercase() == "y" {
                return Ok(stored);
            }
        }

        println!("Enter password:");
        
        let password = read_password()?;
        *password_cache.borrow_mut() = Some(password.clone());

        Ok(password)
    }
}

pub fn create_crypto() -> Box<dyn Crypto> {
    #[cfg(feature = "gpg")]
    {
        if cfg!(target_os = "linux") && GpgCrypto::is_available() {
            if cfg!(debug_assertions) {
                println!("==={}===", "Using GPG encryption".bold());
            }
            return Box::new(GpgCrypto::default());
        }
    }
    if cfg!(debug_assertions) {
        println!("==={}===", "Using built-in encryption".bold());
    }
    Box::new(NativeCrypto::default())
}