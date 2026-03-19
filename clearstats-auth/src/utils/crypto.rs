use argon2::{Argon2, Params, PasswordHasher};
use axum::http::StatusCode;
use base64::{Engine, engine::general_purpose::STANDARD_NO_PAD};
use argon2::password_hash::Salt;
use rand::{RngExt, distr::Alphanumeric};

use shared_lib::error::{AppServerResult, ServerErrorResponse};

pub fn hash_password(password: &str, salt: &str) -> AppServerResult<String> {
    let base64_salt = STANDARD_NO_PAD.encode(salt);
    let password_salt = Salt::from_b64(&base64_salt).map_err(|err| {
        let error_msg = "Salt error";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;

    let params = Params::new(19456, 2, 1, Some(16)).map_err(|err| {
        let error_msg = "Invalid params";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    match argon2.hash_password(password.as_bytes(), password_salt) {
        Ok(password_hash) => Ok(password_hash.to_string()),
        Err(e) => {
            let err =
                ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1234, e.to_string());
            Err(err)
        }
    }
}

pub fn generate_salt() -> String {
    let salt: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    salt
}

pub fn generate_verification_code() -> String {
    let s: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect();
    s
}

pub fn generate_reset_code() -> String {
    let s: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .map(|c| c.to_ascii_uppercase())
        .collect();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tracing_test::traced_test]
    #[test]
    fn test_hash_password_001() -> AppServerResult {
        let expected =
            "$argon2id$v=19$m=19456,t=2,p=1$bXljcmVkZW50aWFsc2FsdA$cv3Vv1J5kBhPMOBZkHCQAw";
        let actual = hash_password("password", "mycredentialsalt")?;

        assert_eq!(actual, expected);
        Ok(())
    }
}
