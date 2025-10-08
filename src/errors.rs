use std::{error, fmt, io, num::ParseIntError, string::FromUtf8Error, time::SystemTimeError};

use hmac::digest::InvalidLength;

use crate::prelude::Colorize;

pub enum AppError {
    Io(io::Error),
    ParseIntError(ParseIntError),
    StorageLoad(String),
    InvalidInput(String),
    FailedTOTP(String),
    Encrypt(String),
    InvalidData,
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::Io(err) => format!("I/O error | {err}").error(),
            AppError::ParseIntError(err) => format!("Parse int error | {err}").error(),
            AppError::StorageLoad(msg) => format!("Storage error | {msg}").error(),
            AppError::InvalidInput(msg) => format!("Invalid input | {msg}").error(),
            AppError::FailedTOTP(msg) => format!("TOTP error | {msg}").error(),
            AppError::Encrypt(msg) => format!("Encrypt error | {msg}").error(),
            AppError::InvalidData => "Invalid data | Decrypted data is not valid UTF-8".error(),
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

impl From<FromUtf8Error> for AppError {
    fn from(_value: FromUtf8Error) -> Self {
        AppError::InvalidData
    }
}