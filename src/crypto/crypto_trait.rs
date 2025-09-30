use std::path::Path;

use rpassword::read_password;

use crate::prelude::{AppError, Totp};

pub trait Crypto {
    fn get_extension_files(&self) -> &str;

    fn encrypting(&self, path_to_file: &Path) -> Result<(), AppError>;

    fn decrypting(&self, path_to_file: &Path) -> Result<(), AppError>;

    fn get_secret(&self) -> Result<String, AppError> {
        println!("Insert TOTP secret:");
        let secret = read_password()?;
        Totp::generate(&secret)?; //just to validate secret
        Ok(secret)
    }

    fn get_password(&self) -> Result<String, AppError> {
        println!("Enter password:");
        let password = read_password()?;
        Ok(password)
    }
}