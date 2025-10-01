use std::{num::NonZeroU32, path::Path};

use ring::{aead, pbkdf2, rand::{SystemRandom, SecureRandom}};

use crate::{prelude::{AppError, Crypto}};

pub struct RingCrypto;

impl RingCrypto {
    const PBKDF2_ITERATIONS: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(100_000) };

    fn derive_key(password: &str, salt: &[u8; 16]) -> aead::LessSafeKey {
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            Self::PBKDF2_ITERATIONS,
            salt,
            password.as_bytes(),
            &mut key,
        );
        aead::LessSafeKey::new(aead::UnboundKey::new(&aead::AES_256_GCM, &key).unwrap())
    }
}

impl Crypto for RingCrypto {
    fn get_extension_files(&self) -> &str {
        "enc"
    }

    fn encrypting(&self, path_to_file: &Path) -> Result<(), AppError> {
        let secret = self.get_secret()?;
        let password = self.get_password()?;

        //generate random salt and nonce
        let rng = SystemRandom::new();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt)?;
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce)?;
        
        //encrypt
        let key = Self::derive_key(&password, &salt);
        let mut data = secret.as_bytes().to_vec();
        key.seal_in_place_append_tag(
            aead::Nonce::assume_unique_for_key(nonce),
            aead::Aad::empty(),
            &mut data,
        )?;
        
        //save: salt + nonce + ciphertext
        let mut file_data = Vec::new();
        file_data.extend_from_slice(&salt);
        file_data.extend_from_slice(&nonce);
        file_data.extend_from_slice(&data);
        
        std::fs::write(path_to_file, &file_data)?;
        Ok(())
    }

    fn decrypting(&self, _path_to_file: &Path) -> Result<(), AppError> {
        Err(AppError::FailedTOTP("TODO decrypting".into()))
    }
}