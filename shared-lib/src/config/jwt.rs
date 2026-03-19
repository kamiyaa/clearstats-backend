use std::time::Duration;

use serde::Deserialize;

#[derive(Clone, Debug, Default)]
pub struct JwtTokenConfig {
    pub secret: String,
    pub expiration: Duration,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JwtTokenConfigRaw {
    pub secret: String,
    pub expiration: u64,
}

impl From<JwtTokenConfigRaw> for JwtTokenConfig {
    fn from(raw: JwtTokenConfigRaw) -> Self {
        Self {
            secret: raw.secret,
            expiration: Duration::new(raw.expiration, 0),
        }
    }
}
