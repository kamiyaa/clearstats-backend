use reqwest::{Method, StatusCode};
use serde::Serialize;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    integrations::stripe::client::auth::create_authenticated_request,
    utils::request::reqwest_send_to_server_error,
};

use super::super::STRIPE_API_ENDPOINT;

#[derive(Clone, Debug, Serialize)]
pub struct MeterEventPayload {
    pub stripe_customer_id: String,
    pub value: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RequestBody {
    pub event_name: String,
    pub timestamp: String,
    pub payload: MeterEventPayload,
}

/// Sends a usage meter event
pub async fn handler(api_key: &str, req: &RequestBody) -> AppServerResult {
    let api_url = format!("{STRIPE_API_ENDPOINT}/v2/billing/meter_events");
    let resp = create_authenticated_request(api_key, Method::POST, &api_url)
        .json(&req)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    let status_code = resp.status();
    if !status_code.is_success() {
        let body = resp.text().await.map_err(|err| {
            tracing::error!(?err, "Failed to parse error response body");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
        })?;
        let err = ServerErrorResponse::new(status_code, 1000, body);
        return Err(err);
    }
    Ok(())
}
