use axum::http::Method;

use crate::error::AppServerResult;
use crate::integrations::gcp::{GCP_STORAGE_API_URL, create_authenticated_request};
use crate::utils::request::reqwest_send_to_server_error;

pub async fn handler(access_token: &str, bucket: &str, file_name: &str) -> AppServerResult {
    let api_url = format!("{GCP_STORAGE_API_URL}/storage/v1/b/{bucket}/o/{file_name}");
    create_authenticated_request(access_token, Method::DELETE, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    Ok(())
}
