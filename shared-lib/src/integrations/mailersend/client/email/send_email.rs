use reqwest::{Method, StatusCode};
use serde::Serialize;

use crate::error::{AppServerResult, ServerErrorResponse};
use crate::integrations::mailersend::auth::create_authenticated_request;
use crate::integrations::mailersend::types::{SendEmailFrom, SendEmailTo, SendEmailVariable};
use crate::utils::request::reqwest_send_to_server_error;

#[derive(Clone, Debug, Default, Serialize, PartialEq)]
pub struct RequestBody {
    pub from: SendEmailFrom,
    pub to: Vec<SendEmailTo>,
    pub subject: Option<String>,
    pub text: String,
    pub html: String,
    pub variables: Vec<SendEmailVariable>,
}

pub async fn handler(api_key: &str, data: &RequestBody) -> AppServerResult {
    let api_url = "https://api.mailersend.com/v1/email";

    tracing::debug!(
        from = data.from.email,
        to = ?data.to,
        "Sending email"
    );

    let resp = create_authenticated_request(api_key, Method::POST, api_url)
        .json(&data)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    let status_code = resp.status();
    if !status_code.is_success() {
        let body = resp.text().await.map_err(|err| {
            tracing::error!(?err, "Failed to parse error response body");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
        })?;
        tracing::error!(api_url, ?status_code, body, "Request Failed",);
        let err = ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, body);
        return Err(err);
    }
    Ok(())
}
