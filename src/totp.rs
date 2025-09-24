use hmac::{Hmac, Mac};
use sha1::Sha1;
use base32::Alphabet;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

use crate::prelude::AppError;

type HmacSha1 = Hmac<Sha1>;

pub struct Totp;

impl Totp {
    const TIME_STEP: u64 = 30;
    const CODE_LENGTH: usize = 6;

    pub fn display(secret: &str) -> Result<(), AppError> {
        loop {
            let (code, remaining) = Self::generate(secret)?;
            let progress = 30 - remaining;
            let progress_bar = "█".repeat(progress as usize) + &"░".repeat(remaining as usize);
            
            print!("\rCode: {code} | {progress_bar} | Time remaining: {remaining:2}s");
            std::io::stdout().flush()?;
            
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn generate(secret: &str) -> Result<(String, u64), AppError> {
        let secret = base32::decode(Alphabet::Rfc4648 { padding: false }, &secret.trim().to_uppercase())
            .ok_or_else(|| AppError::FailedTOTP("Invalid base32 secret".into()))?;

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let time_counter = current_time / Self::TIME_STEP;
        let time_remaining = Self::TIME_STEP - (current_time % Self::TIME_STEP);
        
        let message = time_counter.to_be_bytes();

        let mut mac = HmacSha1::new_from_slice(&secret)?;
        mac.update(&message);
        let hmac_hash = mac.finalize().into_bytes();

        let offset = (hmac_hash[hmac_hash.len() - 1] & 0x0F) as usize;
        
        let binary_code = ((hmac_hash[offset] as u32 & 0x7F) << 24)
            | ((hmac_hash[offset + 1] as u32 & 0xFF) << 16)
            | ((hmac_hash[offset + 2] as u32 & 0xFF) << 8)
            | (hmac_hash[offset + 3] as u32 & 0xFF);

        let width = Self::CODE_LENGTH;
        let code = binary_code % 10u32.pow(width as u32);
        
        Ok((format!("{code:0>width$}"), time_remaining))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_secret() {
        let result = Totp::generate("JBSWY3DPEHPK3PXP");
        assert!(result.is_ok());

        let (code, remaining) = result.unwrap();
        assert_eq!(code.len(), Totp::CODE_LENGTH);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
        assert!(remaining <= Totp::TIME_STEP);
    }

    #[test]
    fn test_invalid_secret() {
        let result = Totp::generate("INVALID123!");
        assert!(result.is_err());
    }
}