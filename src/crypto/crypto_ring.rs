use std::path::Path;

use crate::{prelude::{AppError, Crypto}};

pub struct CryptoRing;

impl Crypto for CryptoRing {
    fn get_extension_files(&self) -> &str {
        "enc"
    }

    fn encrypting(&self, _path_to_file: &Path) -> Result<(), AppError> {
        Ok(())
    }

    fn decrypting(&self, _path_to_file: &Path) -> Result<(), crate::prelude::AppError> {
        Ok(())
    }
}