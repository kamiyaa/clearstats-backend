use axum::http::StatusCode;
use axum::{Json, extract::State};

use serde::{Deserialize, Serialize};

use shared_lib::config::env::Environment;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::integrations::mailersend::client::mock_client::MockMailersendClient;
use shared_lib::integrations::mailersend::client::{MailersendClient, MailersendClientTrait};
use shared_lib::utils::time::get_secs_since_epoch;

use crate::AppState;
use crate::database::user_credential::reset_password;
use crate::database::user_recovery::{delete_reset_code, fetch_password_reset_code};
use crate::utils::crypto;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub token: String,
    pub username: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {}

pub async fn handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    if app_state.config.environment == Environment::Local {
        tracing::warn!("Using mock client");
        let mock_client = MockMailersendClient::default();
        mock_client
            .responses
            .send_email
            .lock()
            .await
            .push_back(Ok(()));
        return _handler(app_state, payload, &mock_client).await;
    }

    let mailersend_client = MailersendClient::new(app_state.config.mailersend_api_key.clone());
    return _handler(app_state, payload, &mailersend_client).await;
}

pub async fn _handler(
    app_state: AppState,
    payload: RequestBody,
    _mailersend_client: &impl MailersendClientTrait,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let RequestBody {
        token,
        username,
        new_password,
    } = payload;

    let db_manager = app_state.get_db_manager();

    let sql_res = fetch_password_reset_code::run_query(db_manager, &username, &token)
        .await
        .map_err(|err| {
            let error_msg = "Failed to reset password";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?
        .ok_or_else(|| {
            let error_msg = "Reset password token not found";
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string())
        })?;

    let now = get_secs_since_epoch()?;
    if sql_res.expires_at < now {
        let error_msg = "Password reset link has expired";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    let new_salt = crypto::generate_salt();
    let new_password_hash = crypto::hash_password(&new_password, &new_salt)?;

    let sql_query = reset_password::SqlQuery {
        user_id: sql_res.user_id,
        new_password_hash,
        new_salt,
    };
    reset_password::run_query(db_manager, &sql_query)
        .await
        .map_err(|err| {
            let error_msg = "Failed to reset password";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

    delete_reset_code::run_query(db_manager, sql_res.user_id)
        .await
        .map_err(|err| {
            let error_msg = "Failed to clean up password reset tokens";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

    /*
    TODO: Send email has been reset email
    let server_msg = ServerMessage::PasswordUpdated {
        email: sql_data.email,
        verification_code,
    };
    let _ = app_state.message_tx.send(server_msg).await;
     */

    let resp = ResponseBody {};
    Ok(ServerSuccessResponse::new(resp))
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use pretty_assertions::assert_eq;
    use shared_lib::database::DatabasePool;
    use shared_lib::error::AppServerResult;
    use shared_lib::test_utils::test_request;

    use super::super::router;
    use super::*;
    use crate::database::user_credential::fetch_user_by_username;
    use crate::test_utils::{
        self,
        test_server::{TestServer, setup_test_server},
    };

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users")),
        fixtures(
            path = "../../../../fixtures/recovery/fetch_reset_code/001",
            scripts("test_data")
        )
    )]
    async fn test_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer {
            app, db_manager, ..
        } = setup_test_server(&config, pool, router());
        // Setup Done

        let body = RequestBody {
            token: "RBRDOE".to_string(),
            username: "jeffbezos88".to_string(),
            new_password: "Password2".to_string(),
        };
        let api_url = "/user/password_reset".to_string();
        let request = test_request::JsonRequestParams {
            app: app.clone(),
            method: Method::POST,
            api_url,
            jwt: None,
            body: body.clone(),
        };
        let response = test_request::send_json_request(request).await;
        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::OK);

        let user = fetch_user_by_username::run_query(&db_manager, "jeffbezos88")
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
        let password_hash = crypto::hash_password("Password2", &user.salt)?;
        assert_eq!(password_hash, user.password_hash);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users")),
        fixtures(
            path = "../../../../fixtures/recovery/fetch_reset_code/001",
            scripts("test_data")
        )
    )]
    async fn test_002_invalid_code(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer {
            app, db_manager, ..
        } = setup_test_server(&config, pool, router());
        // Setup Done

        let body = RequestBody {
            token: "ABCDEF".to_string(),
            username: "jeffbezos88".to_string(),
            new_password: "Password2".to_string(),
        };
        let api_url = "/user/password_reset".to_string();
        let request = test_request::JsonRequestParams {
            app: app.clone(),
            method: Method::POST,
            api_url,
            jwt: None,
            body: body.clone(),
        };
        let response = test_request::send_json_request(request).await;
        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::NOT_FOUND);

        let user = fetch_user_by_username::run_query(&db_manager, "jeffbezos88")
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

        let password_hash = crypto::hash_password("password", &user.salt)?;
        assert_eq!(password_hash, user.password_hash);

        Ok(())
    }
}
