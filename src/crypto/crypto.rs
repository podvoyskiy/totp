use std::path::Path;

use crate::{prelude::AppError};

pub trait Crypto {
    fn get_extension_files(&self) -> &str;

    fn encrypting(&self, path_to_file: &Path) -> Result<(), AppError>;

    fn decrypting(&self, path_to_file: &Path) -> Result<(), AppError>;
}