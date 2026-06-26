use std::{fmt::Display, fs::remove_file, path::Path};
use chrono::{DateTime, Local, Utc};
use rsntp::SntpClient;

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

    pub fn check_time() -> Result<()> {
        let client = SntpClient::new();
        let sync_result = client
            .synchronize("pool.ntp.org")
            .map_err(|err| AppError::Ntp(err.to_string()))?;

        let ntp_utc: DateTime<Utc> = sync_result
            .datetime()
            .into_chrono_datetime()
            .map_err(|err| AppError::Ntp(err.to_string()))?;
        
        let local_utc= Local::now().with_timezone(&Utc);

        let diff = (local_utc - ntp_utc).num_seconds().abs();

        if diff > 5 {
            let diff_sign = if local_utc > ntp_utc { "ahead" } else { "behind" };
            println!("{}", format!("System clock is {diff_sign} by {diff} seconds. TOTP codes may not work correctly.").yellow());
        }

        Ok(())
    }
}

