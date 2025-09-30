use std::path::Path;

use crate::{prelude::{AppError, Crypto}};

pub struct RingCrypto;

impl Crypto for RingCrypto {
    fn get_extension_files(&self) -> &str {
        "enc"
    }

    fn encrypting(&self, _path_to_file: &Path) -> Result<(), AppError> {
        Err(AppError::FailedTOTP("TODO encrypting".into()))
    }

    fn decrypting(&self, _path_to_file: &Path) -> Result<(), AppError> {
        Err(AppError::FailedTOTP("TODO decrypting".into()))
    }
}