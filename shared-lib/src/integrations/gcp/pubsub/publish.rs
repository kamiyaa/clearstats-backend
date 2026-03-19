use serde::Serialize;

use crate::error::AppServerResult;
use crate::integrations::gcp::create_authenticated_request;
use crate::utils::request::reqwest_send_to_server_error;
use crate::utils::response::try_parse_json_response;

use super::{PublishMessageRequest, PublishMessageResponse};

pub async fn publish_message<T: Serialize>(
    access_token: &str,
    project_id: &str,
    topic: &str,
    message: &T,
) -> AppServerResult<PublishMessageResponse> {
    let topic_format = format!("projects/{project_id}/topics/{topic}");
    let api_url = format!("https://pubsub.googleapis.com/v1/{topic_format}:publish");
    let json_body = PublishMessageRequest {
        messages: vec![message],
    };
    let resp = create_authenticated_request(access_token, reqwest::Method::POST, &api_url)
        .json(&json_body)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}
