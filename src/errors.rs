use std::{error, fmt, io, num::ParseIntError, time::SystemTimeError};

use hmac::digest::InvalidLength;

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    ParseIntError(ParseIntError),
    StorageLoad(String),
    InvalidInput(String),
    FailedTOTP(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(err) => write!(f, "I/O error: {err}"),
            AppError::ParseIntError(err) => write!(f, "Parse int error: {err}"),
            AppError::StorageLoad(msg) => write!(f, "Storage error: {msg}"),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            AppError::FailedTOTP(msg) => write!(f, "TOTP error: {msg}"),
        }
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
        AppError::ParseIntError(value)
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