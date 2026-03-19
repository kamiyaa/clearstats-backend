use axum::extract::State;
use axum::http::StatusCode;
use axum_extra::extract::CookieJar;
use serde::Serialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::{COOKIE_REFRESH_TOKEN, RefreshToken, UserClaims};

use crate::AppState;
use crate::database::user_credential::fetch_user_by_username;
use crate::utils::jwt;

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {
    pub access_token: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let cookie = jar.get(COOKIE_REFRESH_TOKEN).ok_or_else(|| {
        let error_msg = "Not authenticated";
        ServerErrorResponse::new(StatusCode::UNAUTHORIZED, 1234, error_msg.to_string())
    })?;
    let claims = RefreshToken::from_str(
        cookie.value(),
        app_state.config.get_jwt_refresh_token_secret().as_bytes(),
    )?;

    let db_manager = app_state.get_db_manager();
    let user = fetch_user_by_username::run_query(db_manager, &claims.username)
        .await
        .map_err(|err| {
            let error_msg = "Failed to fetch user credentials";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?
        .ok_or_else(|| {
            let error_msg = "Incorrect email or password";
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string())
        })?;

    let user_jwt = UserClaims {
        user_id: user.id,
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        verified: user.email_verified > 0,
        icon_hash: user.icon_hash,
    };

    let token = jwt::generate_access_token_from_config(
        app_state.config.get_jwt_token_secret(),
        app_state.config.jwt_token_lifetime,
        user_jwt,
    )?;
    Ok(ServerSuccessResponse::new(ResponseBody {
        access_token: token,
    }))
}
