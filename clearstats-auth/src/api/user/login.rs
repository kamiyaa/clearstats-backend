use axum::http::StatusCode;
use axum::{Json, extract::State};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::UserClaims;

use crate::AppState;
use crate::database::user_credential::fetch_user_by_email;
use crate::utils::jwt::generate_refresh_token_cookie;
use crate::utils::{crypto, jwt};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestBody {
    pub email: String,
    pub password: String,
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
    let RequestBody { email, password } = payload;

    let db_manager = app_state.get_db_manager();
    let user = fetch_user_by_email::run_query(db_manager, &email)
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
            let error_msg = "Incorrect email or password";
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string())
        })?;

    let password_hash = crypto::hash_password(&password, &user.salt).map_err(|_| {
        let error_msg = "Failed to process password";
        tracing::error!("{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;

    if password_hash != user.password_hash {
        let error_msg = "Incorrect email or password";
        let err = ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, error_msg.to_string());
        return Err(err);
    }

    let user_jwt = UserClaims {
        user_id: user.id,
        username: user.username,
        first_name: user.first_name,
        last_name: user.last_name,
        verified: user.email_verified > 0,
        icon_hash: user.icon_hash,
    };

    let access_token = jwt::generate_access_token_from_config(
        app_state.config.get_jwt_token_secret(),
        app_state.config.jwt_token_lifetime,
        user_jwt.clone(),
    )?;
    let refresh_token = jwt::generate_refresh_token(
        app_state.config.get_jwt_refresh_token_secret(),
        app_state.config.jwt_refresh_token_lifetime,
        user_jwt,
    )?;
    let refresh_cookie = generate_refresh_token_cookie(refresh_token, &app_state.config);

    let resp = ResponseBody { access_token };
    Ok((jar.add(refresh_cookie), ServerSuccessResponse::new(resp)))
}

fn sanitize_input(payload: RequestBody) -> RequestBody {
    RequestBody {
        email: payload.email.to_lowercase(),
        ..payload
    }
}

#[cfg(test)]
mod tests {
    use axum::http::Method;
    use pretty_assertions::assert_eq;

    use reqwest::header::SET_COOKIE;
    use shared_lib::database::DatabasePool;
    use shared_lib::error::AppServerResult;
    use shared_lib::test_utils::test_request;

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

        let api_url = "/user/login".to_string();
        let body = RequestBody {
            email: "alice@clearstats.dev".to_string(),
            password: "password".to_string(),
        };
        let request = test_request::JsonRequestParams {
            app: app.clone(),
            method: Method::POST,
            api_url,
            jwt: None,
            body,
        };
        let response = test_request::send_json_request(request).await;

        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::OK);

        let headers = response.headers();
        let refresh_cookie = headers.get(SET_COOKIE);
        assert!(refresh_cookie.is_some());

        Ok(())
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(
            path = "../../../../fixtures",
            scripts("0010_init_users")
        )
    )]
    async fn test_002_incorrect_password(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestServer { app, .. } = setup_test_server(&config, pool, router());
        // Setup Done

        let api_url = "/user/login".to_string();
        let body = RequestBody {
            email: "alice@clearstats.dev".to_string(),
            password: "password2".to_string(),
        };
        let request = test_request::JsonRequestParams {
            app: app.clone(),
            method: Method::POST,
            api_url,
            jwt: None,
            body,
        };
        let response = test_request::send_json_request(request).await;

        let resp_status = response.status();
        assert_eq!(resp_status, StatusCode::NOT_FOUND);

        Ok(())
    }
}
