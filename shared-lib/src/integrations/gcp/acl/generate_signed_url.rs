use axum::http::Method;
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use sha2::Digest;

use crate::error::{AppServerResult, ServerErrorResponse};
use crate::integrations::gcp::GCP_STORAGE_API_URL;
use crate::integrations::gcp::acl::sign_blob;
use crate::integrations::gcp::util::CanoncialRequest;

#[derive(Clone, Debug)]
pub struct RequestBody {
    pub method: Method,
    pub object_path: String,
    pub now: DateTime<Utc>,
    pub service_account_email: String,
    pub expires_secs: u64,
}

/// https://docs.cloud.google.com/storage/docs/access-control/signed-urls
pub async fn handler(access_token: &str, req: &RequestBody) -> AppServerResult<String> {
    let RequestBody {
        object_path, now, ..
    } = req;

    let datestamp = now.format("%Y%m%d").to_string();
    let iso_date = now.format("%Y%m%dT%H%M%SZ").to_string();

    // Canonical request
    let canonical_request = CanoncialRequest::from(req);
    let canonical_request_str = canonical_request.to_string();

    tracing::debug!(canonical_request_str, "Canonical Request");

    // String to sign
    let hash = sha2::Sha256::digest(canonical_request_str.as_bytes());
    let hex_hash = hex::encode(hash);
    let string_to_sign = format!(
        r#"GOOG4-RSA-SHA256
{iso_date}
{datestamp}/auto/storage/goog4_request
{hex_hash}"#
    );

    tracing::debug!(string_to_sign, "String to sign");

    let req = sign_blob::RequestBody {
        service_account_email: req.service_account_email.clone(),
        payload: string_to_sign.as_bytes(),
    };

    let sig_resp = sign_blob::handler(access_token, &req).await?;
    let sig_encoded = encode_signature(&sig_resp.signed_blob)?;
    tracing::debug!(?sig_resp, ?sig_encoded, "Signed blob");

    let signed_url = format!(
        "{GCP_STORAGE_API_URL}{object_path}?{}&X-Goog-Signature={sig_encoded}",
        canonical_request.canonical_querystring,
    );

    Ok(signed_url)
}

fn encode_signature(s: &str) -> AppServerResult<String> {
    // First, we decode it to binary, then we encode it to hex
    let decoded_sig = STANDARD.decode(s).map_err(|err| {
        let error_msg = "Failed to decode signature";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            error_msg.to_string(),
        )
    })?;
    Ok(hex::encode(&decoded_sig))
}
