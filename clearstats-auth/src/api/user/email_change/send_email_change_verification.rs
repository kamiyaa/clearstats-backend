use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;
use shared_lib::utils::time::get_secs_since_epoch;

use crate::database;
use crate::database::user_credential::check_email_exists;
use crate::queue::types::ServerMessage;
use crate::{AppState, utils::crypto};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub new_email: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let claims =
        AccessToken::from_header_map(headers, app_state.config.get_jwt_token_secret().as_bytes())?;

    let RequestBody { new_email, .. } = payload;

    let db_manager = app_state.get_db_manager();
    let email_exists = check_email_exists::run_query(db_manager, &new_email)
        .await
        .map_err(|err| {
            let error_msg = "Failed to validate email";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;
    if email_exists > 0 {
        let err = ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            "A user with this email already exists".to_string(),
        );
        return Err(err);
    }

    let verification_code = crypto::generate_verification_code();
    let created_at = get_secs_since_epoch()?;

    database::email_change_request::upsert_email_change_request::run_query(
        db_manager,
        &new_email,
        claims.user.user_id,
        &verification_code,
        created_at,
    )
    .await
    .map_err(|err| {
        let error_msg = "Failed to generate verification code";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;

    let server_msg = ServerMessage::SendVerificationEmail {
        email: new_email,
        verification_code,
    };
    let _ = app_state.message_tx.send(server_msg).await;

    // TODO: verified should be false once email verifications work

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
    use shared_lib::test_utils::test_user::TestUser;

    use super::super::router;
    use super::*;
    use crate::database::email_change_request::fetch_email_change_verification_code;
    use crate::queue::types::ServerMessage;
    use crate::test_utils::{
        self,
        test_server::{TestServer, setup_test_server},
    };

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users"))
    )]
    async fn test_change_email_request_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();

        let TestServer {
            app,
            db_manager,
            mut rx,
            ..
        } = setup_test_server(&config, pool, router());

        // Setup Done

        let user_jwt = TestUser::Alice.generate_jwt(config.get_jwt_token_secret().as_bytes())?;
        let new_email = "alice_new@indaggo.com".to_string();
        let body = RequestBody {
            new_email: new_email.clone(),
        };
        let api_url = "/email/change_request".to_string();

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

        // check server message was created
        let msg = rx.recv().await.expect("No server message received");
        match msg {
            ServerMessage::SendVerificationEmail {
                email,
                verification_code,
            } => {
                let actual = ServerMessage::SendVerificationEmail {
                    email: email.clone(),
                    verification_code: verification_code.clone(),
                };
                let expected = ServerMessage::SendVerificationEmail {
                    email: new_email.to_string(),
                    verification_code: verification_code.clone(),
                };
                assert_eq!(actual, expected);

                let code_exists = fetch_email_change_verification_code::run_query(
                    &db_manager,
                    &email,
                    &verification_code,
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

                assert_eq!(code_exists, true);
            }
        }

        Ok(())
    }
}
