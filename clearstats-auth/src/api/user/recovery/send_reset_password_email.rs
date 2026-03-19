use axum::http::StatusCode;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

use shared_lib::config::env::Environment;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::integrations::mailersend::NO_REPLY_EMAIL;
use shared_lib::integrations::mailersend::client::mock_client::MockMailersendClient;
use shared_lib::integrations::mailersend::client::{MailersendClient, MailersendClientTrait};
use shared_lib::integrations::mailersend::types::MailerSendEmailRequestBuilder;
use shared_lib::utils::time::get_secs_since_epoch;

use crate::AppState;
use crate::database;
use crate::database::user_credential::fetch_user_by_email;
use crate::utils::crypto;

const SUBJECT: &str = "ClearStats Account Password Reset";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestBody {
    pub email: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<()>> {
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
    mailersend_client: &impl MailersendClientTrait,
) -> AppServerResult<ServerSuccessResponse<()>> {
    let RequestBody { email } = payload;

    let db_manager = app_state.get_db_manager();
    let user = fetch_user_by_email::run_query(db_manager, &email)
        .await
        .map_err(|err| {
            let error_msg = "Failed to fetch user email";
            tracing::error!(?err, email, "{error_msg}");
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
    let verification_code = crypto::generate_reset_code();
    let expires_at = get_secs_since_epoch()? + 3600;

    database::user_recovery::insert_password_reset_code::run_query(
        db_manager,
        user.id,
        &verification_code,
        expires_at,
    )
    .await
    .map_err(|err| {
        let error_msg = "Failed to insert password reset code";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;

    let username = user.username;
    let clearstats_lab_url = app_state.config.clearstats_lab_url.as_str();
    let email_content = format!(
        "Here is your password reset link: {clearstats_lab_url}/user/password_reset?token={verification_code}&username={username}"
    );

    let data =
        MailerSendEmailRequestBuilder::new(NO_REPLY_EMAIL.to_string(), "ClearStats".to_string())
            .to_email(email)
            .subject(SUBJECT.to_string())
            .content(email_content)
            .build();

    mailersend_client.send_email(&data).await?;
    Ok(ServerSuccessResponse::new(()))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use sqlx::prelude::FromRow;

    use shared_lib::database::manager::DatabaseManagerTrait;
    use shared_lib::database::tables::user::{TABLE_USER_CREDENTIAL, TABLE_USER_PASSWORD_RESET};
    use shared_lib::database::{DatabasePool, DatabaseResult};
    use shared_lib::error::AppServerResult;
    use shared_lib::integrations::mailersend::client::email::send_email;
    use shared_lib::integrations::mailersend::types::{SendEmailFrom, SendEmailTo};

    use super::*;
    use crate::test_utils::{
        self,
        test_server::{TestAppState, setup_app_state},
    };

    #[derive(Clone, Debug, FromRow)]
    struct UserPasswordResetCode {
        pub code: String,
    }
    async fn fetch_user_password_reset_code(
        pool: &DatabasePool,
        email: &str,
    ) -> DatabaseResult<UserPasswordResetCode> {
        let sql_query = format!(
            "
            SELECT
                code
            FROM
                {TABLE_USER_PASSWORD_RESET} pw_reset
            INNER JOIN
                {TABLE_USER_CREDENTIAL} user_cred
            ON
                pw_reset.user_id = user_cred.id
            WHERE
                user_cred.email = ?
        ;"
        );
        let sql_res = sqlx::query_as(&sql_query)
            .bind(email)
            .fetch_one(pool)
            .await?;
        Ok(sql_res)
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users",))
    )]
    async fn test_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestAppState { app_state, .. } = setup_app_state(&config, pool);
        // Setup Done

        let body = RequestBody {
            email: "alice@clearstats.dev".into(),
        };

        let mock_client = MockMailersendClient::default();
        mock_client
            .responses
            .send_email
            .lock()
            .await
            .push_back(Ok(()));
        let resp = _handler(app_state.clone(), body.clone(), &mock_client).await?;
        assert!(resp.body.ok);

        let email_req = mock_client
            .requests
            .send_email
            .lock()
            .await
            .pop_front()
            .expect("Missing email request");

        let expected = send_email::RequestBody {
            to: vec![SendEmailTo {
                email: body.email.clone(),
            }],
            from: SendEmailFrom {
                name: "ClearStats".into(),
                email: NO_REPLY_EMAIL.into(),
            },
            subject: Some(SUBJECT.to_string()),
            ..email_req.clone()
        };
        assert_eq!(expected, email_req);

        let password_code =
            fetch_user_password_reset_code(app_state.db_manager.get_database_pool(), &body.email)
                .await
                .expect("Failed to fetch password reset code");
        assert!(email_req.text.contains(&password_code.code));
        Ok(())
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users",))
    )]
    async fn test_002_email_does_not_exist(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestAppState { app_state, .. } = setup_app_state(&config, pool);
        // Setup Done

        let body = RequestBody {
            email: "unknown-user@clearstats.dev".into(),
        };

        let mock_client = MockMailersendClient::default();
        let resp = _handler(app_state, body.clone(), &mock_client)
            .await
            .err()
            .expect("Expected an error");
        assert_eq!(resp.status_code, StatusCode::NOT_FOUND);

        assert!(mock_client.requests.send_email.lock().await.is_empty());
        Ok(())
    }

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users",))
    )]
    async fn test_003_multiple_reset_requests(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestAppState { app_state, .. } = setup_app_state(&config, pool);
        // Setup Done

        let body = RequestBody {
            email: "alice@clearstats.dev".into(),
        };

        let mock_client = MockMailersendClient::default();
        for _ in 0..2 {
            mock_client
                .responses
                .send_email
                .lock()
                .await
                .push_back(Ok(()));
        }
        let resp = _handler(app_state.clone(), body.clone(), &mock_client).await?;
        assert!(resp.body.ok);
        let resp = _handler(app_state.clone(), body.clone(), &mock_client).await?;
        assert!(resp.body.ok);

        // pop the first request, because the second request should overwrite it
        mock_client
            .requests
            .send_email
            .lock()
            .await
            .pop_front()
            .expect("Missing email request");
        let email_req = mock_client
            .requests
            .send_email
            .lock()
            .await
            .pop_front()
            .expect("Missing email request");

        let expected = send_email::RequestBody {
            to: vec![SendEmailTo {
                email: body.email.clone(),
            }],
            from: SendEmailFrom {
                name: "ClearStats".into(),
                email: NO_REPLY_EMAIL.into(),
            },
            subject: Some(SUBJECT.to_string()),
            ..email_req.clone()
        };
        assert_eq!(expected, email_req);

        let password_code =
            fetch_user_password_reset_code(app_state.db_manager.get_database_pool(), &body.email)
                .await
                .expect("Failed to fetch password reset code");
        assert!(email_req.text.contains(&password_code.code));
        Ok(())
    }
}
