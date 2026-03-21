use cookie::{Cookie, SameSite};
use shared_lib::config::env::Environment;
use shared_lib::database::DatabaseInteger;
use shared_lib::utils::time::get_secs_since_epoch;

use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header};

use shared_lib::error::{AppServerResult, ServerErrorResponse};
use shared_lib::types::jwt::{self, COOKIE_REFRESH_TOKEN, UserClaims};

use crate::config::AppConfig;

pub fn generate_access_token_from_config(
    secret: &str,
    lifetime: DatabaseInteger,
    jwt_data: UserClaims,
) -> AppServerResult<String> {
    let key = secret.as_bytes();

    // Valid for 3 days
    let valid_duration = lifetime;
    let exp = get_secs_since_epoch()? + valid_duration;

    let claims = jwt::AccessToken {
        exp,
        user: jwt_data,
    };

    generate_access_token(key, &claims)
}

pub fn generate_access_token(key: &[u8], claims: &jwt::AccessToken) -> AppServerResult<String> {
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
        .map_err(|err| {
        let error_msg = "Failed to encode JWT token";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;
    Ok(token)
}

pub fn generate_refresh_token(
    secret: &str,
    lifetime: DatabaseInteger,
    jwt_data: UserClaims,
) -> AppServerResult<String> {
    let key = secret.as_bytes();

    // Valid for 3 days
    let valid_duration = lifetime;
    let exp = get_secs_since_epoch()? + valid_duration;

    let claims = jwt::RefreshToken {
        user_id: jwt_data.user_id,
        username: jwt_data.username,
        exp,
    };
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
        .map_err(|err| {
        let error_msg = "Failed to encode JWT token";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;
    Ok(token)
}

pub fn generate_refresh_token_cookie(token: String, config: &AppConfig) -> Cookie<'static> {
    let max_age = cookie::time::Duration::new(config.jwt_refresh_token_lifetime as i64, 0);
    let mut refresh_cookie = Cookie::new(COOKIE_REFRESH_TOKEN, token);
    refresh_cookie.set_http_only(true);
    refresh_cookie.set_same_site(SameSite::Lax);
    refresh_cookie.set_path("/refresh");
    refresh_cookie.set_max_age(max_age);
    if config.environment == Environment::Local {
        refresh_cookie.set_secure(false);
    } else {
        refresh_cookie.set_secure(true);
    }
    refresh_cookie
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use shared_lib::{error::AppServerResult, types::jwt::UserClaims};

    use super::*;

    #[test]
    fn test_generate_access_token() -> AppServerResult {
        let user_claims = UserClaims {
            user_id: 10000,
            username: "User123".to_string(),
            first_name: "John".to_string(),
            last_name: "Smith".to_string(),
            verified: false,
            icon_hash: None,
        };

        let key = b"12345678";
        let jwt_data = jwt::AccessToken {
            user: user_claims,
            exp: DatabaseInteger::MAX - 1,
        };
        let jwt_token = generate_access_token(key, &jwt_data)?;

        assert_eq!(
            jwt_token,
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyIjp7InVzZXJfaWQiOjEwMDAwLCJ1c2VybmFtZSI6IlVzZXIxMjMiLCJmaXJzdF9uYW1lIjoiSm9obiIsImxhc3RfbmFtZSI6IlNtaXRoIiwidmVyaWZpZWQiOmZhbHNlLCJpY29uX2hhc2giOm51bGx9LCJleHAiOjE4NDQ2NzQ0MDczNzA5NTUxNjE0fQ.MgvgnPRCdY-egVDPj3tUlaYWtVpES6j2ei3CdnVsSYQ"
        );

        Ok(())
    }
}
