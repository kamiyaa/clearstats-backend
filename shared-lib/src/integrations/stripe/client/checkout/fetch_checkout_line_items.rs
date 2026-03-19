use reqwest::Method;

use crate::{
    error::AppServerResult,
    integrations::stripe::{client::auth::create_authenticated_request, types::ListObject},
    utils::{request::reqwest_send_to_server_error, response::try_parse_json_response},
};

use super::super::STRIPE_API_ENDPOINT;

pub async fn handler(api_key: &str, checkout_id: &str) -> AppServerResult<ListObject> {
    let api_url = format!("{STRIPE_API_ENDPOINT}/v1/checkout/sessions/{checkout_id}/line_items");
    let resp = create_authenticated_request(api_key, Method::GET, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}
