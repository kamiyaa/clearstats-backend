use axum::http::{HeaderMap, StatusCode};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::{AccessToken, UserClaims};

use crate::AppState;
use crate::database::user_credential::{
    fetch_user_by_user_id, fetch_user_email, update_email_verified,
};
use crate::database::user_verification::fetch_verification_code;
use crate::utils::jwt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub verification_code: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {
    pub access_token: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let claims = AccessToken::from_header_map_unverified(
        headers,
        app_state.config.get_jwt_token_secret().as_bytes(),
    )?;

    let RequestBody { verification_code } = payload;

    let db_manager = app_state.get_db_manager();
    let user_email = fetch_user_email::run_query(db_manager, claims.user.user_id)
        .await
        .map_err(|err| {
            let error_msg = "Failed to fetch user email";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?
        .ok_or_else(|| {
            let error_msg = "Email not found";
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string())
        })?;

    let code_exists =
        fetch_verification_code::run_query(db_manager, &user_email, &verification_code)
            .await
            .map_err(|err| {
                let error_msg = "Failed to fetch verification code";
                tracing::error!(?err, "{error_msg}");
                ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1234,
                    error_msg.to_string(),
                )
            })?;

    if !code_exists {
        let error_msg = "Invalid verification code";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    let rows_affected = update_email_verified::run_query(db_manager, &user_email, true)
        .await
        .map_err(|err| {
            let error_msg = "Failed to verify email";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

    if rows_affected == 0 {
        let error_msg = "Failed to verify user";
        return Err(ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            error_msg.to_string(),
        ));
    }

    // TODO: for security, we should prob ask them to login
    // again instead of just giving them a jwt token
    let user = fetch_user_by_user_id::run_query(db_manager, claims.user.user_id)
        .await
        .map_err(|err| {
            let error_msg = "Failed to fetch user";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?
        .ok_or_else(|| {
            let error_msg = "User not found";
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string())
        })?;

    let user_jwt = UserClaims {
        user_id: user.id,
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        verified: user.email_verified,
        icon_hash: user.icon_hash,
    };

    let access_token = jwt::generate_access_token_from_config(
        app_state.config.get_jwt_token_secret(),
        app_state.config.jwt_token_lifetime,
        user_jwt.clone(),
    )?;

    let resp = ResponseBody { access_token };
    Ok(ServerSuccessResponse::new(resp))
}

#[cfg(test)]
mod tests {

    use axum::http::Method;
    use pretty_assertions::assert_eq;

    use shared_lib::database::DatabasePool;
    use shared_lib::error::AppServerResult;
    use shared_lib::test_utils::test_request;
    use shared_lib::test_utils::test_user::TestUser;

    use crate::database::user_credential::fetch_user_by_username;
    use crate::test_utils::{
        self,
        test_server::{TestServer, setup_test_server},
    };

    use super::super::router;
    use super::*;

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users")),
        fixtures(
            path = "../../../../fixtures/verification/fetch_verification_code/001",
            scripts("test_data")
        )
    )]
    async fn test_verify_email_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer {
            app, db_manager, ..
        } = setup_test_server(&config, pool, router());
        // Setup Done

        let user = fetch_user_by_username::run_query(&db_manager, "daniel")
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

        assert!(user.email_verified == 0);

        let user_jwt = TestUser::Daniel.generate_jwt(config.get_jwt_token_secret().as_bytes())?;

        let body = RequestBody {
            verification_code: "D3JTJ4".to_string(),
        };
        let api_url = "/email/verify_email".to_string();

        let request = test_request::JsonRequestParams {
            app: app.clone(),
            method: Method::POST,
            api_url,
            jwt: Some(user_jwt.clone()),
            body: body.clone(),
        };
        let response = test_request::send_json_request(request).await;

        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::OK);

        let user = fetch_user_by_username::run_query(&db_manager, "daniel")
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

        assert!(user.email_verified != 0);

        Ok(())
    }
}
