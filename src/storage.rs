use std::{fs, path::PathBuf};
use crate::prelude::AppError;
use directories::ProjectDirs;

pub struct Storage {
    pub services: Vec<PathBuf>,
}

impl Storage {
    const GPG_EXTENSION: &str = "gpg";

    pub fn load() -> Result<Self, AppError> {
        let project_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .ok_or(AppError::StorageLoad("Failed to get config directory".into()))?;

        let config_dir = project_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).map_err(|e| AppError::StorageLoad(format!("Failed to create config directory: {e}")))?;
            println!("Created config directory: {}", config_dir.display());
        }

        if !config_dir.is_dir() {
            return Err(AppError::StorageLoad(format!("Config path is not a directory: {}", config_dir.display())));
        }

        let gpg_files: Vec<PathBuf> = fs::read_dir(config_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file() && path.extension().is_some_and(|ext| ext == Self::GPG_EXTENSION)
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