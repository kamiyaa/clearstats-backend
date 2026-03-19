use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::integrations::mailersend::NO_REPLY_EMAIL;
use shared_lib::integrations::mailersend::client::{MailersendClient, MailersendClientTrait};
use shared_lib::integrations::mailersend::types::MailerSendEmailRequestBuilder;
use shared_lib::types::jwt::AccessToken;
use shared_lib::utils::time::get_secs_since_epoch;

use crate::database;
use crate::database::user_credential::fetch_user_email;
use crate::{AppState, utils::crypto};

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> AppServerResult<ServerSuccessResponse<()>> {
    let mailersend_client = MailersendClient::new(app_state.config.mailersend_api_key.clone());
    let verification_code = crypto::generate_verification_code();
    _handler(app_state, headers, &mailersend_client, &verification_code).await
}

async fn _handler(
    app_state: AppState,
    headers: HeaderMap,
    mailersend_client: &impl MailersendClientTrait,
    verification_code: &str,
) -> AppServerResult<ServerSuccessResponse<()>> {
    let claims = AccessToken::from_header_map_unverified(
        headers,
        app_state.config.get_jwt_token_secret().as_bytes(),
    )?;
    let db_manager = app_state.get_db_manager();
    let user_email = fetch_user_email::run_query(db_manager, claims.user.user_id)
        .await
        .map_err(|err| {
            let error_msg = "Failed to fetch email";
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

    let created_at = get_secs_since_epoch()?;

    database::user_verification::upsert_verification_code::run_query(
        db_manager,
        &user_email,
        verification_code,
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

    let subject = "Email Verification Code".to_string();
    let email_content = format!("Your verification code is: {verification_code}");
    let data =
        MailerSendEmailRequestBuilder::new(NO_REPLY_EMAIL.to_string(), "ClearStats".to_string())
            .to_email(user_email)
            .subject(subject)
            .content(email_content)
            .build();
    mailersend_client.send_email(&data).await?;

    Ok(ServerSuccessResponse::new(()))
}

#[cfg(test)]
mod tests {
    use reqwest::header::AUTHORIZATION;
    use shared_lib::database::DatabasePool;
    use shared_lib::integrations::mailersend::client::mock_client::MockMailersendClient;
    use shared_lib::test_utils::test_user::TestUser;

    use crate::test_utils::test_server::{TestAppState, setup_app_state};
    use crate::test_utils::{self};

    use super::*;

    #[tracing_test::traced_test]
    #[sqlx::test(
        migrator = "shared_lib::database::DEFAULT_MIGRATOR",
        fixtures(path = "../../../../../fixtures", scripts("0010_init_users")),
        fixtures(
            path = "../../../../fixtures/recovery/fetch_reset_code/001",
            scripts("test_data")
        )
    )]
    async fn test_reset_verification_code_001_happy_path(pool: DatabasePool) -> AppServerResult {
        // Setup
        let config = test_utils::config::get_test_config();
        let TestAppState { app_state, .. } = setup_app_state(&config, pool);
        // Setup Done

        let user_jwt = TestUser::Alice.generate_jwt(config.get_jwt_token_secret().as_bytes())?;

        let mut headers = HeaderMap::new();
        headers.append(AUTHORIZATION, format!("Bearer {user_jwt}").parse().unwrap());

        let mock_client = MockMailersendClient::default();
        mock_client
            .responses
            .send_email
            .lock()
            .await
            .push_back(Ok(()));

        let verification_code = "123456";
        _handler(app_state, headers, &mock_client, verification_code).await?;

        let expected = MailerSendEmailRequestBuilder::new(
            NO_REPLY_EMAIL.to_string(),
            "ClearStats".to_string(),
        )
        .to_email("alice@clearstats.dev".into())
        .subject("Email Verification Code".into())
        .content(format!("Your verification code is: {verification_code}"))
        .build();

        let actual = mock_client
            .requests
            .send_email
            .lock()
            .await
            .pop_front()
            .expect("Request is missing");

        assert_eq!(actual, expected);
        Ok(())
    }
}
