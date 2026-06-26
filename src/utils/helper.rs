use std::{fmt::Display, fs::remove_file, path::Path};
use chrono::{DateTime, Utc};
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

    pub fn check_time() -> Result<i64> {
        Self::check_time_with(&RealTimeProvider)
    }

    fn check_time_with<T: TimeProvider>(time_provider: &T) -> Result<i64> {
        let client = SntpClient::new();
        let sync_result = client
            .synchronize("pool.ntp.org")
            .map_err(|err| AppError::Ntp(err.to_string()))?;

        let ntp_utc: DateTime<Utc> = sync_result
            .datetime()
            .into_chrono_datetime()
            .map_err(|err| AppError::Ntp(err.to_string()))?;
        
        let local_utc= time_provider.now_utc();

        let diff = (local_utc - ntp_utc).num_seconds().abs();

        if diff > 5 {
            let diff_sign = if local_utc > ntp_utc { "ahead" } else { "behind" };
            println!("{}", format!("System clock is {diff_sign} by {diff} seconds. TOTP codes may not work correctly.").yellow());
        }

        Ok(diff)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_with_real_time() {
        let diff = Helper::check_time_with(&RealTimeProvider).unwrap();
        assert!(diff <= 5);
    }

    #[test]
    fn test_with_mock_time() {
        let provider = MockTimeProvider::new(10);
        let diff = Helper::check_time_with(&provider).unwrap();
        assert!(diff > 5);
    }
}
