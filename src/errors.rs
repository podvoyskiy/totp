use std::{error, fmt, io, num::{ParseIntError, TryFromIntError}, string::FromUtf8Error, time::SystemTimeError};

use hmac::digest::InvalidLength;

use crate::prelude::*;

pub enum AppError {
    Io(io::Error),
    IntError(String),
    StorageLoad(String),
    InvalidInput(String),
    FailedTOTP(String),
    Encrypt(String),
    InvalidData,
    FileNameError,
    JsonError(String),
    RemoveFile(String),
    Ntp(String),
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::Io(err) => format!("I/O error | {err}").red(),
            AppError::IntError(msg) => msg.red(),
            AppError::StorageLoad(msg) => format!("Storage error | {msg}").red(),
            AppError::InvalidInput(msg) => format!("Invalid input | {msg}").red(),
            AppError::FailedTOTP(msg) => format!("TOTP error | {msg}").red(),
            AppError::Encrypt(msg) => format!("Encrypt error | {msg}").red(),
            AppError::InvalidData => "Invalid data | Decrypted data is not valid UTF-8".red(),
            AppError::FileNameError => "FileNameError | Invalid file name".red(),
            AppError::JsonError(msg) => format!("Json error | {msg}").red(),
            AppError::RemoveFile(path_to_file) => format!("Failed to delete file: {path_to_file}").red(),
            AppError::Ntp(msg) => format!("NTP error | {msg}").red(),
        };
        write!(f, "{message}")
    }
}

impl error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<ParseIntError> for AppError {
    fn from(value: ParseIntError) -> Self {
        AppError::IntError(format!("Parse error: {value}"))
    }
}

impl From<TryFromIntError> for AppError {
    fn from(value: TryFromIntError) -> Self {
        AppError::IntError(format!("Conversion error: {value}"))
    }
}

impl From<SystemTimeError> for AppError {
    fn from(value: SystemTimeError) -> Self {
        AppError::FailedTOTP(value.to_string())
    }
}

impl From<InvalidLength> for AppError {
    fn from(value: InvalidLength) -> Self {
        AppError::FailedTOTP(value.to_string())
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(_value: FromUtf8Error) -> Self {
        AppError::InvalidData
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::JsonError(value.to_string())
    }
}