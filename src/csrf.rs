use bytemuck::{Pod, Zeroable};
use data_encoding::BASE64URL;
use hmac::{Hmac, Mac};
use once_cell::sync::Lazy;
use rand::{thread_rng, RngCore};
use sha2::Sha256;
use std::env;
use time::OffsetDateTime;

/// Generate a new token.
pub fn generate_token() -> String {
    Token::new().as_string()
}

/// Verify that a token is valid.
pub fn verify_token(token: impl AsRef<str>) -> bool {
    if let Ok(t) = BASE64URL.decode(token.as_ref().as_bytes()) {
        if let Ok(t) = bytemuck::try_from_bytes::<Token>(&t) {
            return t.is_valid();
        }
    }

    false
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
struct Token {
    /// Just some random bytes to make token generation non-deterministic.
    salt: [u8; 32],

    /// Timestamp when the token was generated. This can be used to validate a
    /// token's age.
    timestamp: i64,

    /// Signature of the previous fields authenticating the token.
    mac: [u8; 32],
}

impl Token {
    /// Generate a new token.
    fn new() -> Self {
        Self::with_timestamp(OffsetDateTime::now_utc())
    }

    fn with_timestamp(timestamp: OffsetDateTime) -> Self {
        let mut salt = [0; 32];
        thread_rng().fill_bytes(&mut salt);

        let timestamp = timestamp.unix_timestamp();

        let mut hmac = create_hmac();
        hmac.update(&salt);
        hmac.update(bytemuck::bytes_of(&timestamp));

        Self {
            salt,
            timestamp,
            mac: hmac.finalize().into_bytes().into(),
        }
    }

    /// Verify that the token is valid.
    fn is_valid(&self) -> bool {
        let mut valid = true;

        // First validate the MAC
        let mut hmac = create_hmac();
        hmac.update(&self.salt);
        hmac.update(bytemuck::bytes_of(&self.timestamp));
        valid &= hmac.verify_slice(&self.mac).is_ok();

        // Validate age
        let now = OffsetDateTime::now_utc().unix_timestamp();
        let age_in_seconds = now - self.timestamp;

        valid &= age_in_seconds >= 5;
        valid &= age_in_seconds < 600;

        valid
    }

    fn as_string(&self) -> String {
        BASE64URL.encode(bytemuck::bytes_of(self))
    }
}

fn create_hmac() -> Hmac<Sha256> {
    static KEY: Lazy<Vec<u8>> = Lazy::new(get_hmac_key);

    Hmac::new_from_slice(KEY.as_slice()).unwrap()
}

fn get_hmac_key() -> Vec<u8> {
    if let Ok(key) = env::var("CSRF_TOKEN_KEY") {
        key.into_bytes()
    } else {
        log::warn!("no CSRF token key configured, generating a random one");

        let mut key = vec![0; 64];
        thread_rng().fill_bytes(&mut key);

        key
    }
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use super::*;

    #[test]
    fn random_string_is_invalid() {
        assert!(!verify_token("hello world"));
    }

    #[test]
    fn brand_new_token_is_invalid() {
        assert!(!verify_token(generate_token()));
    }

    #[test]
    fn token_is_valid_after_5_seconds() {
        let token =
            Token::with_timestamp(OffsetDateTime::now_utc() - Duration::seconds(5)).as_string();

        assert!(verify_token(token));
    }

    #[test]
    fn token_is_invalid_after_10_minutes() {
        let token =
            Token::with_timestamp(OffsetDateTime::now_utc() - Duration::minutes(10)).as_string();

        assert!(!verify_token(token));
    }
}
