use std::{fs, path::PathBuf};
use crate::prelude::AppError;

const GPG_EXTENSION: &str = "gpg";

pub struct Storage {
    pub services: Vec<PathBuf>,
}

impl Storage {
    pub fn load() -> Result<Self, AppError> {
        let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));

        let config_dir = xdg_dirs.get_config_home().ok_or(AppError::StorageLoad("Failed to get config directory".into()))?;

        if !config_dir.exists() || !config_dir.is_dir() {
            return Err(AppError::StorageLoad("Config directory does not exist".into()));
        }

        let gpg_files: Vec<PathBuf> = fs::read_dir(&config_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file() && path.extension().is_some_and(|ext| ext == GPG_EXTENSION)
            })
            .collect();

        if gpg_files.is_empty() {
            return Err(AppError::StorageLoad(format!("No .gpg files found in {}", config_dir.display())));
        }

        Ok(Self { services: gpg_files})
    }
}

#[cfg(test)]
mod test {
    use super::Storage;

    #[test]
    fn storage_load() {
        assert!(Storage::load().is_ok())
    }
}