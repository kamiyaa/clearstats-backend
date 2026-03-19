use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use serde::Serialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::email_change_request::{
    delete_email_change_request, fetch_user_pending_email,
};

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let claims = AccessToken::from_header_map_unverified(
        headers,
        app_state.config.get_jwt_token_secret().as_bytes(),
    )?;

    let db_manager = app_state.get_db_manager();
    let pending_email = fetch_user_pending_email::run_query(db_manager, claims.user.user_id)
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

    delete_email_change_request::run_query(db_manager, &pending_email)
        .await
        .map_err(|err| {
            let error_msg = "Failed to clean up email change request";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

    let resp = ResponseBody {};
    Ok(ServerSuccessResponse::new(resp))
}

#[cfg(test)]
mod tests {

    use axum::http::Method;
    use pretty_assertions::assert_eq;

    use crate::database::email_change_request::fetch_email_change_verification_code;
    use crate::database::user_credential::fetch_user_by_username;
    use crate::test_utils::{
        self,
        test_server::{TestServer, setup_test_server},
    };
    use shared_lib::database::DatabasePool;
    use shared_lib::error::AppServerResult;
    use shared_lib::test_utils::test_request;
    use shared_lib::test_utils::test_user::TestUser;

    use super::super::router;
    use super::*;

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users")),
        fixtures(
            path = "../../../../fixtures/email_change/fetch_email_change_verification_code/001",
            scripts("test_data")
        )
    )]
    async fn test_cancel_email_change_001_happy_path(pool: DatabasePool) -> AppServerResult {
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

        let user_jwt = TestUser::Alice.generate_jwt(config.get_jwt_token_secret().as_bytes())?;

        let api_url = "/email/change_request".to_string();

        let request = test_request::EmptyRequestParams {
            app: app.clone(),
            method: Method::DELETE,
            api_url,
            jwt: Some(user_jwt.clone()),
        };
        let response = test_request::send_empty_request(request).await;

        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::OK);

        let code_exists = fetch_email_change_verification_code::run_query(
            &db_manager,
            "newALICE@clearstats.dev",
            "D3JTJ4",
        )
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

        assert_eq!(code_exists, false);

        Ok(())
    }
}
