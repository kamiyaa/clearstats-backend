use axum::http::{HeaderMap, StatusCode, header::AUTHORIZATION};

use jsonwebtoken::{DecodingKey, TokenData, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::{
    database::{DatabaseInteger}, error::{AppServerResult, ServerErrorResponse}, utils::time::get_secs_since_epoch
};

pub const COOKIE_REFRESH_TOKEN: &str = "REFRESH_TOKEN";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserClaims {
    pub user_id: DatabaseInteger,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub verified: bool,
    pub icon_hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub user: UserClaims,
    pub exp: DatabaseInteger,
}

impl AccessToken {
    pub fn from_header_map(header_map: HeaderMap, secret: &[u8]) -> AppServerResult<Self> {
        let access_token = Self::from_header_map_unverified(header_map, secret)?;
        if !access_token.user.verified {
            let err = ServerErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                1234,
                "Email not verified".to_string(),
            );
            return Err(err);
        }
        Ok(access_token)
    }

    pub fn from_header_map_unverified(
        header_map: HeaderMap,
        secret: &[u8],
    ) -> AppServerResult<Self> {
        let jwt_token = get_bearer(&header_map)?;
        let token = Self::from_str(jwt_token, secret)?;
        let now = get_secs_since_epoch()?;
        if token.exp < now {
            let err = ServerErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                1234,
                "Token expired".to_string(),
            );
            return Err(err);
        }
        Ok(token)
    }

    pub fn from_str(token: &str, secret: &[u8]) -> AppServerResult<Self> {
        let token_data: TokenData<Self> = decode(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|err| {
            let error_msg = "Invalid token";
            tracing::debug!(?err, ?token, "{error_msg}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string())
        })?;
        let access_token = token_data.claims;
        Ok(access_token)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RefreshToken {
    pub user_id: DatabaseInteger,
    pub username: String,
    pub exp: DatabaseInteger,
}

impl RefreshToken {
    pub fn from_header_map(header_map: HeaderMap, secret: &[u8]) -> AppServerResult<Self> {
        let jwt_token = get_bearer(&header_map)?;
        Self::from_str(jwt_token, secret)
    }

    pub fn from_str(token: &str, secret: &[u8]) -> AppServerResult<Self> {
        let token_data: TokenData<Self> = decode(
            token,
            &DecodingKey::from_secret(secret),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map_err(|err| {
            let error_msg = "Invalid token";
            tracing::debug!(token, "{error_msg}: {err}");
            ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string())
        })?;
        let access_token = token_data.claims;
        Ok(access_token)
    }
}

pub fn get_bearer(header_map: &HeaderMap) -> AppServerResult<&str> {
    let header_value = header_map.get(AUTHORIZATION).ok_or_else(|| {
        let error_msg = "Missing Authorization headers";
        tracing::debug!("{error_msg}");
        ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string())
    })?;

    let bearer_token = header_value.to_str().map_err(|err| {
        let error_msg = "Failed to decode headers";
        tracing::debug!("{error_msg}: {err}");
        ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string())
    })?;

    let jwt_token = bearer_token.strip_prefix("Bearer ").ok_or_else(|| {
        let error_msg = "Invalid Authorization header format";
        tracing::debug!("{error_msg}");
        ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string())
    })?;
    Ok(jwt_token)
}
