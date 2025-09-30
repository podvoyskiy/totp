use std::{fs, path::PathBuf};
use crate::prelude::AppError;
use directories::ProjectDirs;
use regex::Regex;

pub struct Storage {
    pub config_dir: PathBuf,
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

        Ok(Self { config_dir: config_dir.to_path_buf(),  services: gpg_files})
    }

    pub fn find_service_by_name(&self, service_name: &str) -> Option<&PathBuf> {
        self.services.iter().find(|path| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy() == service_name)
                .unwrap_or(false)
        })
    }

    pub fn validate_file_name(service_name: &str) -> bool {
        if service_name.len() > 255 {
            return false;
        }
        let valid_chars = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        valid_chars.is_match(service_name)
    }

    pub fn get_service_path(&self, service_name: &str) -> PathBuf {
        self.config_dir.join(format!("{}.{}", service_name, Self::GPG_EXTENSION))
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