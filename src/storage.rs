use std::{fs::{self, File}, path::{Path, PathBuf}};
use crate::prelude::*;
use directories::ProjectDirs;

pub struct Storage {
    pub crypto: Box<dyn Crypto>,
    pub services: Vec<PathBuf>,
    pub config_dir: PathBuf,
    pub backup_file: PathBuf,
}

impl Storage {
    pub fn new(crypto: Box<dyn Crypto>) -> Result<Self> {
        let project_dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .ok_or(AppError::StorageLoad("Failed to get config dir".into()))?;

        let config_dir = Self::check_dir(project_dirs.config_dir())?;

        let services: Vec<PathBuf> = fs::read_dir(&config_dir)?
            .filter_map(std::result::Result::ok)
            .map(|entry| entry.path())
            .filter(|path| {
                path.is_file() && path.extension().is_some_and(|ext| ext == crypto.get_extension_files())
            })
            .collect();

        let backup_file = Self::check_dir(project_dirs.cache_dir())?.join("backup.json");

        Ok(Self {
            crypto, 
            services,
            config_dir,
            backup_file,
        })
    }

    pub fn get_service_path(&self, service_name: &str) -> PathBuf {
        self.config_dir.join(format!("{}.{}", service_name, self.crypto.get_extension_files()))
    }

    pub fn validate_file_name(service_name: &str) -> bool {
        if service_name.len() > 255 {
            return false;
        }
        service_name.chars().all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-'))
    }

    pub fn delete_service(&self, service_index: usize) -> Result<()> {
        Helper::remove_file(&self.services[service_index])
    }

    pub fn export_services(&self) -> Result<()> {
        let mut services= Vec::new();

        for f in &self.services {
            let service_name = f.file_stem().ok_or(AppError::FileNameError)?.to_string_lossy().to_string();
            println!("{}{}", "current service: ".bold(), service_name.bold());

            let service_secret = self.crypto.decrypting(f)?;

            services.push((service_name, service_secret));
        }

        let json_string = serde_json::to_string_pretty(&services)?;
        std::fs::write(&self.backup_file, json_string)?;

        println!("{}\nBackup file saved to: {}", 
            "Warning: Secrets are stored in plain text!".yellow(), 
            self.backup_file.display()
        );
        Ok(())
    }

    pub fn import_services(&self) -> Result<()> {
        if !self.backup_file.exists() {
            return Err(AppError::InvalidInput(format!("Backup file {} not found", self.backup_file.display())));
        }
        
        let file = File::open(&self.backup_file)?;
        let services: Vec<(String, String)> = serde_json::from_reader(file)?;

        for (service_name, service_secret) in services  {
            println!("{}{}", "current service: ".bold(), service_name.bold());

            if !Storage::validate_file_name(&service_name) {
                return Err(AppError::InvalidInput("incorrect service name".into()));
            }

            self.crypto.encrypting(&self.get_service_path(&service_name), service_secret)?;
        }

        if Helper::confirm("Do you want to delete backup file? (y/n):".yellow()) {
            Helper::remove_file(&self.backup_file)?;
        }

        Ok(())
    }

    fn check_dir(dir: &Path) -> Result<PathBuf> {
        if !dir.exists() {
            fs::create_dir_all(dir).map_err(|e| AppError::StorageLoad(format!("Failed to create dir: {e}")))?;
            println!("Created dir: {}", dir.display());
        }
        if !dir.is_dir() {
            return Err(AppError::StorageLoad(format!("Path is not a dir: {}", dir.display())));
        }
        Ok(dir.to_path_buf())
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::create_crypto;

    use super::Storage;

    #[test]
    fn storage_load() {
        assert!(Storage::new(create_crypto()).is_ok())
    }
}