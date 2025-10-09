use std::{fs, path::PathBuf};
use directories::ProjectDirs;
use totp::prelude::{AppError, Crypto};

pub struct Storage {
    pub crypto: Box<dyn Crypto>,
    pub config_dir: PathBuf,
    pub services: Vec<PathBuf>,
}

impl Storage {
    pub fn new(crypto: Box<dyn Crypto>) -> Result<Self, AppError> {
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

        let services: Vec<PathBuf> = fs::read_dir(config_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file() && path.extension().is_some_and(|ext| ext == "enc")
            })
            .collect();

        Ok(Self {
            crypto, 
            config_dir: config_dir.to_path_buf(), 
            services
        })
    }

    pub fn get_service_path(&self, service_name: &str) -> PathBuf {
        self.config_dir.join(format!("{service_name}.enc"))
    }

    pub fn validate_file_name(service_name: &str) -> bool {
        if service_name.len() > 255 {
            return false;
        }
        service_name.chars().all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-'))
    }
}

#[cfg(test)]
mod test {
    use totp::prelude::NativeCrypto;

    use super::Storage;

    #[test]
    fn storage_load() {
        assert!(Storage::new(Box::new(NativeCrypto)).is_ok())
    }
}