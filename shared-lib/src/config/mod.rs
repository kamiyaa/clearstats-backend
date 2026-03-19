pub mod env;
pub mod jwt;
pub mod service_config;

use axum::http::StatusCode;
use serde::de::DeserializeOwned;

use crate::error::{AppServerResult, ServerErrorResponse};

pub fn parse_str_to_config<T, S>(contents: &str) -> AppServerResult<S>
where
    T: DeserializeOwned,
    S: From<T>,
{
    let config = serde_json::from_str::<T>(contents).map_err(|err| {
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
    })?;
    Ok(S::from(config))
}

pub fn parse_config_or_default<T, S>(contents: &str) -> S
where
    T: DeserializeOwned,
    S: From<T> + std::default::Default,
{
    match parse_str_to_config::<T, S>(contents) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to parse contents: {}", e);
            S::default()
        }
    }
}
