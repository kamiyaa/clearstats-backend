use axum::http::StatusCode;
use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::UserClaims;
use shared_lib::utils::input_validation::validate_username;
use shared_lib::utils::{input_validation, time::get_secs_since_epoch};

use crate::database::user_credential::{check_email_exists, check_username_exists, insert_user};
use crate::database::user_verification::upsert_verification_code;
use crate::queue::types::ServerMessage;
use crate::utils::jwt::generate_refresh_token_cookie;
use crate::{
    AppState,
    utils::{crypto, jwt},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub email: String,
    pub password: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub accept_tos: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ResponseBody {
    pub access_token: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<(CookieJar, ServerSuccessResponse<ResponseBody>)> {
    let payload = sanitize_input(payload);
    validate_input(&payload)?;

    let RequestBody {
        email,
        password,
        first_name,
        last_name,
        username,
        ..
    } = payload;

    let db_manager = app_state.get_db_manager();

    let email_exists = check_email_exists::run_query(db_manager, &email)
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
    let username_exists = check_username_exists::run_query(db_manager, &username)
        .await
        .map_err(|err| {
            let error_msg = "Failed to validate username";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

    if username_exists > 0 {
        let err = ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            "A user with this username already exists".to_string(),
        );
        return Err(err);
    }

    let salt = crypto::generate_salt();
    let password_hash = crypto::hash_password(&password, &salt)?;

    let created_at = get_secs_since_epoch()?;
    let sql_data = insert_user::SqlData {
        email,
        password_hash,
        salt,
        email_verified: false,

        username,
        first_name,
        last_name,

        created_at,
        updated_at: created_at,
    };

    let user_id = insert_user::run_query(db_manager, &sql_data)
        .await
        .map_err(|err| {
            let error_msg = "Failed to register user";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;

    let verification_code = crypto::generate_verification_code();

    upsert_verification_code::run_query(
        db_manager,
        &sql_data.email,
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
        email: sql_data.email,
        verification_code,
    };
    let _ = app_state.message_tx.send(server_msg).await;

    // TODO: verified should be false once email verifications work
    let user_jwt = UserClaims {
        user_id,
        username: sql_data.username,
        first_name: sql_data.first_name,
        last_name: sql_data.last_name,
        verified: false,
        icon_hash: None,
    };
    let access_token = jwt::generate_access_token_from_config(
        app_state.config.get_jwt_token_secret(),
        app_state.config.jwt_token_lifetime,
        user_jwt.clone(),
    )?;
    let refresh_token = jwt::generate_refresh_token(
        app_state.config.get_jwt_refresh_token_secret(),
        app_state.config.jwt_refresh_token_lifetime,
        user_jwt.clone(),
    )?;

    let resp = ResponseBody { access_token };
    let refresh_cookie = generate_refresh_token_cookie(refresh_token, &app_state.config);
    Ok((jar.add(refresh_cookie), ServerSuccessResponse::new(resp)))
}

fn sanitize_input(payload: RequestBody) -> RequestBody {
    RequestBody {
        email: payload.email.trim().to_lowercase(),
        first_name: payload.first_name.trim().to_string(),
        last_name: payload.last_name.trim().to_string(),
        ..payload
    }
}

fn validate_input(payload: &RequestBody) -> AppServerResult {
    if !payload.accept_tos {
        let error_msg = "User must agree to Terms and Conditions";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    if !validate_username(&payload.username) {
        let error_msg = "Username can only contain lowercase letters a-z and '-' '_' '.'";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }
    if payload.username.starts_with("clearstats") {
        let error_msg = "Username cannot start with clearstats, reserved";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    if payload.first_name.is_empty() {
        let error_msg = "First name cannot be empty";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }
    if payload.first_name.len() > 50 {
        let error_msg = "First name too long";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    if payload.last_name.is_empty() {
        let error_msg = "Last name cannot be empty";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }
    if payload.last_name.len() > 50 {
        let error_msg = "Last name too long";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    if payload.password.len() < 8 {
        let error_msg = "Password must be at least 8 characters long";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    if payload.email.len() > 320 {
        let error_msg = "Email too long";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }

    if !input_validation::validate_email(&payload.email) {
        let error_msg = "Email is invalid";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::{Router, http::Method};
    use pretty_assertions::assert_eq;

    use shared_lib::database::DatabasePool;
    use shared_lib::error::{AppServerResult, ServerSuccessBody};
    use shared_lib::test_utils::parse::body_to_json_response;
    use shared_lib::test_utils::test_request;
    use shared_lib::test_utils::test_user::TestUser;

    use super::super::router;
    use super::*;
    use crate::test_utils::{
        self,
        test_server::{TestServer, setup_test_server},
    };

    fn create_register_request(
        app: Router,
        body: RequestBody,
    ) -> test_request::JsonRequestParams<RequestBody> {
        test_request::JsonRequestParams {
            app,
            method: Method::POST,
            api_url: "/user/register".to_string(),
            jwt: None,
            body,
        }
    }

    #[tracing_test::traced_test]
    #[sqlx::test(migrator = "shared_lib::database::DEFAULT_MIGRATOR")]
    async fn test_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer { app, mut rx, .. } = setup_test_server(&config, pool, router());
        // Setup Done

        let user_email = "alice@clearstats.dev";
        let body = RequestBody {
            email: user_email.to_string(),
            password: "password".to_string(),
            username: TestUser::Alice.username().to_string(),
            first_name: "Alice".to_string(),
            last_name: "Wonderland".to_string(),
            accept_tos: true,
        };

        let request = create_register_request(app.clone(), body.clone());
        let response = test_request::send_json_request(request).await;

        let resp_status = response.status();
        let result: ServerSuccessBody<ResponseBody> =
            body_to_json_response(response.into_body()).await?;
        assert_eq!(resp_status, StatusCode::OK);
        assert!(result.ok);

        // check server message was created
        let msg = rx.recv().await.expect("No server message received");
        match msg {
            ServerMessage::SendVerificationEmail {
                email,
                verification_code,
            } => {
                let actual = ServerMessage::SendVerificationEmail {
                    email,
                    verification_code: verification_code.clone(),
                };
                let expected = ServerMessage::SendVerificationEmail {
                    email: user_email.to_string(),
                    verification_code,
                };
                assert_eq!(actual, expected);
            }
        }
        Ok(())
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../fixtures", scripts("0010_init_users",))
    )]
    async fn test_002_username_already_exists(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer { app, .. } = setup_test_server(&config, pool, router());
        // Setup Done

        let user_email = "alice@clearstats.dev";
        let body = RequestBody {
            email: user_email.to_string(),
            password: "password".to_string(),
            username: TestUser::Alice.username().to_string(),
            first_name: "Alice".to_string(),
            last_name: "Wonderland".to_string(),
            accept_tos: true,
        };

        let request = create_register_request(app.clone(), body.clone());
        let response = test_request::send_json_request(request).await;

        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::BAD_REQUEST);
        Ok(())
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../fixtures", scripts("0010_init_users",))
    )]
    async fn test_003_invalid_username(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer { app, .. } = setup_test_server(&config, pool, router());
        // Setup Done

        let user_email = "alice@clearstats.dev";
        let body = RequestBody {
            email: user_email.to_string(),
            password: "password".to_string(),
            username: "alice asda".to_string(),
            first_name: "Alice".to_string(),
            last_name: "Wonderland".to_string(),
            accept_tos: true,
        };

        let request = create_register_request(app.clone(), body.clone());
        let response = test_request::send_json_request(request).await;

        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::BAD_REQUEST);
        Ok(())
    }
}
