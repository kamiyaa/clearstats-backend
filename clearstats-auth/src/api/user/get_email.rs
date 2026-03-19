use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::user_credential::fetch_user_email;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ResponseBody {
    pub email: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let claims = AccessToken::from_header_map_unverified(
        headers,
        app_state.config.get_jwt_token_secret().as_bytes(),
    )?;

    let db_manager = app_state.get_db_manager();
    let email = fetch_user_email::run_query(db_manager, claims.user.user_id)
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
            let error_msg = "User not found";
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string())
        })?;

    Ok(ServerSuccessResponse::new(ResponseBody { email }))
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use pretty_assertions::assert_eq;

    use shared_lib::database::DatabasePool;
    use shared_lib::error::{AppServerResult, ServerSuccessBody};
    use shared_lib::test_utils::parse::body_to_json_response;
    use shared_lib::test_utils::test_request;
    use shared_lib::test_utils::test_user::TestUser;

    use crate::test_utils::{
        self,
        test_server::{TestServer, setup_test_server},
    };

    use super::super::router;
    use super::*;

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(
            path = "../../../../fixtures",
            scripts("0010_init_users")
        )
    )]
    async fn test_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer { app, .. } = setup_test_server(&config, pool, router());
        // Setup Done

        let user_jwt = TestUser::Charlie.generate_jwt(config.get_jwt_token_secret().as_bytes())?;

        let api_url = "/email".to_string();
        let request = test_request::EmptyRequestParams {
            app: app.clone(),
            method: Method::GET,
            api_url,
            jwt: Some(user_jwt.clone()),
        };
        let response = test_request::send_empty_request(request).await;

        let resp_status = response.status();
        let result: ServerSuccessBody<ResponseBody> =
            body_to_json_response(response.into_body()).await?;
        assert_eq!(resp_status, StatusCode::OK);
        assert!(result.ok);

        let actual = result.result;
        let expected = ResponseBody {
            email: "charlie@clearstats.dev".to_string(),
        };
        assert_eq!(actual, expected);
        Ok(())
    }
}
