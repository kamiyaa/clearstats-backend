use reqwest::{Method, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppServerResult,
    integrations::stripe::client::auth::create_authenticated_request,
    utils::{request::reqwest_send_to_server_error, response::try_parse_json_response},
};

use super::super::STRIPE_API_ENDPOINT;

#[derive(Clone, Debug, Serialize)]
pub struct RequestBody {
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseBody {
    pub id: String,
}

pub async fn handler(api_key: &str, req: &RequestBody) -> AppServerResult<ResponseBody> {
    let api_url = format!("{STRIPE_API_ENDPOINT}/v1/customers");
    let resp = create_authenticated_request(api_key, Method::POST, &api_url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&req)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}
