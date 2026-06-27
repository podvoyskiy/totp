use qrcode::{QrCode, render::unicode};

use crate::prelude::*;

pub fn display(label: &str, secret: &str) -> Result<()> {
    let otpauth = format!("otpauth://totp/{label}?secret={secret}");

    let code = QrCode::new(otpauth).map_err(|e| AppError::Qrcode(e.to_string()))?;

    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{image}");
    println!("{}", "Scan this QR code with your authenticator app to add this service".dimmed());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_qr() {
        assert!(display("label", "JBSWY3DPEHPK3PXP").is_ok());
    }
}
