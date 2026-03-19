use axum::http::Method;

use crate::error::AppServerResult;
use crate::integrations::gcp::bucket::GcpBucketObject;
use crate::integrations::gcp::{self, GCP_STORAGE_API_URL};
use crate::utils::request::reqwest_send_to_server_error;
use crate::utils::response::try_parse_json_response;

pub async fn handler(
    access_token: &str,
    bucket: &str,
    obj_path: &str,
) -> AppServerResult<GcpBucketObject> {
    let api_url = format!("{GCP_STORAGE_API_URL}/storage/v1/b/{bucket}/o/{obj_path}");
    let resp = gcp::create_authenticated_request(access_token, Method::GET, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    try_parse_json_response(resp).await
}
