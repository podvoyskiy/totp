use std::path::Path;

use rpassword::read_password;

use crate::{Totp, AppError};

pub trait Crypto {
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