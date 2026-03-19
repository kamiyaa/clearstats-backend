use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::user_credential::{fetch_user_by_user_id, update_password};
use crate::utils::crypto;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let claims = AccessToken::from_header_map_unverified(
        headers,
        app_state.config.get_jwt_token_secret().as_bytes(),
    )?;

    let RequestBody {
        old_password,
        new_password,
    } = payload;

    let db_manager = app_state.get_db_manager();

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

    let old_password_hash = crypto::hash_password(&old_password, &user.salt)?;
    if old_password_hash != user.password_hash {
        let error_msg = "Incorrect old password";
        let err = ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string());
        return Err(err);
    }

    let new_salt = crypto::generate_salt();
    let new_password_hash = crypto::hash_password(&new_password, &new_salt)?;

    let sql_data = update_password::SqlData {
        user_id: user.id,
        salt: new_salt,
        password_hash: new_password_hash,
    };
    let res = update_password::run_query(db_manager, &sql_data)
        .await
        .map_err(|err| {
            let error_msg = "Failed to update password";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;
    if res == 0 {
        let error_msg = "Password incorrect";
        let err = ServerErrorResponse::new(StatusCode::BAD_REQUEST, 1234, error_msg.to_string());
        return Err(err);
    }
    Ok(ServerSuccessResponse::new(ResponseBody {}))
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
        fixtures(path = "../../../../fixtures", scripts("0010_init_users"))
    )]
    async fn test_change_password_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer {
            app, db_manager, ..
        } = setup_test_server(&config, pool, router());
        // Setup Done

        let user_jwt = TestUser::Alice.generate_jwt(config.get_jwt_token_secret().as_bytes())?;

        let body = RequestBody {
            old_password: "password".to_string(),
            new_password: "ChangedPassword123".to_string(),
        };
        let api_url = "/user/change_password".to_string();

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

        let user = fetch_user_by_username::run_query(&db_manager, "alice")
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

        let password_hash = crypto::hash_password("ChangedPassword123", &user.salt)?;

        assert_eq!(password_hash, user.password_hash);

        Ok(())
    }
}
