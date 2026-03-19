use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    error::AppServerResult,
    integrations::gcp,
    utils::{request::reqwest_send_to_server_error, response::try_parse_json_response},
};

const IAM_API_URL: &str = "https://iamcredentials.googleapis.com";

#[derive(Clone, Debug)]
pub struct RequestBody<'a> {
    pub service_account_email: String,
    pub payload: &'a [u8],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    pub key_id: String,
    pub signed_blob: String,
}

pub async fn handler<'a>(
    access_token: &str,
    req: &RequestBody<'a>,
) -> AppServerResult<ResponseBody> {
    let RequestBody {
        service_account_email,
        payload,
    } = req;

    let payload_b64 = STANDARD.encode(payload);

    let api_url =
        format!("{IAM_API_URL}/v1/projects/-/serviceAccounts/{service_account_email}:signBlob",);

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct SignBlobRequest {
        pub payload: String,
    }

    let json_body = SignBlobRequest {
        payload: payload_b64,
    };
    let resp = gcp::create_authenticated_request(access_token, Method::POST, &api_url)
        .json(&json_body)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}
