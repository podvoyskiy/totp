use std::path::PathBuf;

use crate::{prelude::{AppError, Crypto}};

pub struct CryptoRing;

impl Crypto for CryptoRing {
    fn encrypting(path_to_file: &PathBuf) -> Result<(), AppError> {
        Ok(())
    }

    fn decrypting(path_to_file: &PathBuf) -> Result<(), crate::prelude::AppError> {
        Ok(())
    }
}