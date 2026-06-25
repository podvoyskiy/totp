use std::{fmt::Display, fs::remove_file, path::Path};
use crate::prelude::*;

pub struct Helper;

impl Helper {
    pub fn confirm(prompt: impl Display) -> bool {
        println!("{prompt}");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        input.trim().to_lowercase() == "y"
    }

    pub fn remove_file(path: &Path) -> Result<()> {
        remove_file(path)
            .inspect(|()| println!("{}", format!("Successfully deleted file: {}", path.display()).cyan()))
            .map_err(|_| AppError::RemoveFile(path.display().to_string()))
    }
}

