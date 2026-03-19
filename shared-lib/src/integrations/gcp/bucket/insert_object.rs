use axum::http::Method;
use serde::Deserialize;

use crate::error::AppServerResult;
use crate::integrations::gcp::{GCP_STORAGE_API_URL, create_authenticated_request};
use crate::utils::request::reqwest_send_to_server_error;
use crate::utils::response::try_parse_json_response;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct ResponseBody {
    pub kind: String,
    pub id: String,
    #[serde(rename = "selfLink")]
    pub self_link: String,
    #[serde(rename = "mediaLink")]
    pub media_link: String,
    pub name: String,
    pub bucket: String,
    pub generation: String,
    pub metageneration: String,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    #[serde(rename = "storageClass")]
    pub storage_class: String,
    pub size: String,
    #[serde(rename = "md5Hash")]
    pub md5_hash: String,
    pub crc32c: String,
    pub etag: String,
    #[serde(rename = "timeCreated")]
    pub time_created: String,
    pub updated: Option<String>,
    #[serde(rename = "timeStorageClassUpdated")]
    pub time_storage_class_updated: Option<String>,
}

pub async fn handler(
    access_token: &str,
    bucket: &str,
    file_name: &str,
    content_type: Option<&str>,
    obj: Vec<u8>,
    generation: Option<&str>,
) -> AppServerResult<ResponseBody> {
    let mut api_url = format!(
        "{GCP_STORAGE_API_URL}/upload/storage/v1/b/{bucket}/o?uploadType=media&name={file_name}"
    );

    if let Some(generation) = generation {
        api_url.push_str("&ifGenerationMatch=");
        api_url.push_str(generation);
    }

    let mut req = create_authenticated_request(access_token, Method::POST, &api_url)
        .header("Content-length", obj.len());
    if let Some(content_type) = content_type {
        req = req.header("Content-type", content_type);
    }

    let body = obj;
    let resp = req
        .body(body)
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;

    try_parse_json_response(resp).await
}
