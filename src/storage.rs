use std::{fs, path::PathBuf};
use crate::prelude::AppError;
use colored::Colorize;
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

        Ok(Self { services: gpg_files})
    }

    pub fn find_service_by_name(&self, service_name: &str) -> Option<&PathBuf> {
        self.services.iter().find(|path| {
            path.file_stem()
                .map(|stem| stem.to_string_lossy() == service_name)
                .unwrap_or(false)
        })
    }

    pub fn print_help(&self) {
        println!("{}{}{}", "Usage:".yellow().bold(), " totp".blue().bold(), " [OPTION]".blue());
        println!("{}", "Options:".yellow().bold());
        println!("{}               Show this help", "  --help".blue().bold());
        println!("{}                Add new service", "  --add".blue().bold());
        println!("{}       Service selection at start", "  {service_name}".blue().bold());
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