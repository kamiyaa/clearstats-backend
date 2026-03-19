pub mod input_validation;
pub mod request;
pub mod response;
pub mod time;

use axum::http::StatusCode;

use crate::error::{AppServerResult, ServerErrorResponse};

#[cfg(test)]
mod input_validation_test;

/// Get environment variable value
pub fn get_env_var(s: &str) -> AppServerResult<String> {
    std::env::var(s).map_err(|err| {
        let error_msg = format!("Failed to get env var '{s}': {err}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
    })
}

/// Check if environment variable matches given value.
/// If environment variable is not set, then false is returned
pub fn check_env_var_match(env_var: &str, val: &str) -> bool {
    get_env_var(env_var)
        .ok()
        .map(|env_val| env_val == val)
        .unwrap_or(false)
}
