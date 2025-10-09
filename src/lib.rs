mod totp;
mod errors;
mod colorize;
mod native;
mod crypto_trait;

pub use crate::{
    totp::Totp,
    errors::AppError, 
    colorize::Colorize,
    native::NativeCrypto,
    crypto_trait::Crypto
};

pub mod prelude {
    pub use crate::{
        Totp,
        AppError, 
        Colorize,
        NativeCrypto,
        Crypto
    };
}