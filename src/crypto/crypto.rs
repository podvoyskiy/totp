use std::path::PathBuf;

use crate::{prelude::AppError};

pub trait Crypto {
    fn encrypting(path_to_file: &PathBuf) -> Result<(), AppError>;

    fn decrypting(path_to_file: &PathBuf) -> Result<(), AppError>;
}