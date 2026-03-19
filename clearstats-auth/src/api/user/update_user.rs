use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::user_credential::update_user;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
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
        first_name,
        last_name,
        email,
    } = payload;

    let sql_data = update_user::SqlData {
        email,
        user_id: claims.user.user_id,
        first_name,
        last_name,
    };

    let db_manager = app_state.get_db_manager();
    let res = update_user::run_query(db_manager, &sql_data)
        .await
        .map_err(|err| {
            let error_msg = "Failed to update user";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;
    if res == 0 {
        let error_msg = "User not found";
        return Err(ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            error_msg.to_string(),
        ));
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
    async fn test_001_whappy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer {
            app, db_manager, ..
        } = setup_test_server(&config, pool, router());
        // Setup Done

        let user_jwt = TestUser::Alice.generate_jwt(config.get_jwt_token_secret().as_bytes())?;

        let body = RequestBody {
            first_name: Some("Allison".to_string()),
            last_name: Some("Bobsky".to_string()),
            email: Some("newemail@mail.com".to_string()),
        };
        let api_url = "/user/update".to_string();

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
        assert_eq!("Allison", user.first_name);
        assert_eq!("Bobsky", user.last_name);
        assert_eq!("newemail@mail.com", user.email);

        Ok(())
    }
}
