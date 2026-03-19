use axum::http::Method;

use axum::body::Bytes;
use urlencoding::encode;

use crate::error::AppServerResult;
use crate::integrations::gcp::{self, GCP_STORAGE_API_URL};
use crate::utils::request::{reqwest_send_to_server_error, reqwest_text_to_server_error};
use crate::utils::response::try_parse_json_response;

use super::GcpObjectListResponse;

pub async fn download_object(
    access_token: &str,
    bucket: &str,
    obj_path: &str,
) -> AppServerResult<Bytes> {
    let api_url = format!("{GCP_STORAGE_API_URL}/storage/v1/b/{bucket}/o/{obj_path}?alt=media");
    let resp = gcp::create_authenticated_request(access_token, Method::GET, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    let bytes = resp.bytes().await.map_err(reqwest_text_to_server_error)?;
    Ok(bytes)
}

pub async fn fetch_object(
    access_token: &str,
    bucket: &str,
    obj_path: &str,
) -> AppServerResult<reqwest::Response> {
    let api_url = format!("{GCP_STORAGE_API_URL}/storage/v1/b/{bucket}/o/{obj_path}",);
    let resp = gcp::create_authenticated_request(access_token, Method::GET, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    Ok(resp)
}

/// https://cloud.google.com/storage/docs/json_api/v1/buckets/list
pub async fn fetch_object_list_files(
    access_token: &str,
    bucket: &str,
    prefix: &str,
) -> AppServerResult<GcpObjectListResponse> {
    let api_url =
        format!("{GCP_STORAGE_API_URL}/storage/v1/b/{bucket}/o?projection=noAcl&prefix={prefix}");

    eprintln!("{api_url}");
    let resp = gcp::create_authenticated_request(access_token, Method::GET, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}

pub async fn fetch_object_list_folders(
    access_token: &str,
    bucket: &str,
    prefix: &str,
) -> AppServerResult<reqwest::Response> {
    let glob = format!("{prefix}*/");
    let encoded_glob = encode(&glob);

    let api_url = format!(
        "{GCP_STORAGE_API_URL}/storage/v1/b/{bucket}/o?projection=noAcl&prefix={prefix}&matchGlob={encoded_glob}"
    );
    let resp = gcp::create_authenticated_request(access_token, Method::GET, &api_url)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    Ok(resp)
}
