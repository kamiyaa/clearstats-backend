use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use serde::Serialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::integrations::mailersend::NO_REPLY_EMAIL;
use shared_lib::integrations::mailersend::client::{MailersendClient, MailersendClientTrait};
use shared_lib::integrations::mailersend::types::MailerSendEmailRequestBuilder;
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::user_credential::fetch_user_email;
use crate::utils::crypto;

#[derive(Serialize)]
pub struct ResponseBody {
    success: bool,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let mailersend_client = MailersendClient::new(app_state.config.mailersend_api_key.clone());

    _handler(app_state, headers, &mailersend_client).await
}

async fn _handler(
    app_state: AppState,
    headers: HeaderMap,
    mailersend_client: &impl MailersendClientTrait,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let claims = AccessToken::from_header_map_unverified(
        headers,
        app_state.config.get_jwt_token_secret().as_bytes(),
    )?;

    // TODO: Make sure users can't send more verification
    // emails if they're already verified.

    let db_manager = app_state.get_db_manager();
    let user_email = fetch_user_email::run_query(db_manager, claims.user.user_id)
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

    let verification_code = crypto::generate_verification_code();
    let subject = "Verification Code".to_string();
    let email_content = format!("Your verification code is: {verification_code}");

    let data =
        MailerSendEmailRequestBuilder::new(NO_REPLY_EMAIL.to_string(), "ClearStats".to_string())
            .to_email(user_email)
            .subject(subject)
            .content(email_content)
            .build();

    mailersend_client.send_email(&data).await?;

    let resp = ResponseBody { success: true };
    Ok(ServerSuccessResponse::new(resp))
}

#[cfg(test)]
mod tests {
    // TODO: tests
}
