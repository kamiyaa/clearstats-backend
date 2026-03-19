use axum::http::StatusCode;
use jsonwebtoken::{EncodingKey, Header};

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    types::jwt,
};

pub fn generate_access_token(key: &[u8], claims: &jwt::AccessToken) -> AppServerResult<String> {
    let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(key))
        .map_err(|err| {
        let error_msg = "Failed to encode token";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;
    Ok(token)
}
