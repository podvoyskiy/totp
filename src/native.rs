use std::path::Path;

use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use chacha20poly1305::aead::Aead;
use rand::RngCore;
use sha2::Sha256;
use pbkdf2::pbkdf2;
use hmac::Hmac;

use crate::prelude::*;

pub struct NativeCrypto;

impl NativeCrypto {
    const PBKDF2_ITERATIONS: u32 = 100_000;
}

impl Crypto for NativeCrypto {
    fn encrypting(&self, path_to_file: &Path) -> Result<(), AppError> {
        let secret = self.get_secret()?;
        let password = self.get_password()?;

        println!("{}", "Encrypting...".info());
        
        //generate salt and nonce
        let mut salt = [0u8; 16];
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut salt);
        rand::rng().fill_bytes(&mut nonce_bytes);

        //derivation key with PBKDF2
        let mut key = [0u8; 32];
        let _ = pbkdf2::<Hmac<Sha256>>(
            password.as_bytes(),
            &salt,
            Self::PBKDF2_ITERATIONS,
            &mut key
        );

        //encrypt
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher.encrypt(nonce, secret.as_bytes().as_ref())
            .map_err(|_| AppError::Encrypt("Encryption failed".into()))?;
        
        //save file structure: [SALT(16)][NONCE(12)][ENCRYPTED_DATA]
        let mut file_data = Vec::new();
        file_data.extend_from_slice(&salt);
        file_data.extend_from_slice(&nonce_bytes);
        file_data.extend_from_slice(&ciphertext);
        
        std::fs::write(path_to_file, &file_data)?;

        println!("{}", format!("Successfully encrypted and saved to: {}", path_to_file.display()).success());

        Ok(())
    }

    fn decrypting(&self, path_to_file: &Path) -> Result<(), AppError> {
        let password = self.get_password()?;

        println!("{}", "Decrypting...".info());

        let file_data = std::fs::read(path_to_file)?;
        if file_data.len() < 28 { // salt(16) + nonce(12) = 28
            return Err(AppError::Encrypt("Invalid file format: file too short".into()));
        }

        // extract salt, nonce, ciphertext
        let salt = &file_data[0..16];
        let nonce_bytes = &file_data[16..28];
        let ciphertext = &file_data[28..];

        //restore key with PBKDF2
        let mut key = [0u8; 32];
        let _ = pbkdf2::<Hmac<Sha256>>(
            password.as_bytes(),
            salt,
            Self::PBKDF2_ITERATIONS,
            &mut key
        );

        //decrypt
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let decrypted = cipher.decrypt(nonce, ciphertext)
            .map_err(|_| AppError::Encrypt("Decryption failed - wrong password?".into()))?;
        
        let secret = String::from_utf8(decrypted)?;

        Totp::display(&secret)?;
        Ok(())
    }
}