use reqwest::Method;

use crate::integrations::gcp::acl::generate_signed_url;

pub fn gen_full_file_path(folder: &str, project_path: &str, file_path: &str) -> String {
    let file_path = format!("{folder}/{project_path}/{file_path}");
    file_path
}

pub fn get_content_type(ext: &str) -> &'static str {
    match ext {
        "jpg" => "image/jpg",
        "png" => "image/png",
        "mp4" => "video/mp4",
        "mp3" => "audio/mp3",
        _ => "application/octet-stream",
    }
}

pub const SIGNING_ALGORITHM: &str = "GOOG4-RSA-SHA256";

#[derive(Clone, Debug)]
pub struct CanoncialRequest {
    pub http_verb: Method,
    pub canonical_uri: String,
    pub canonical_querystring: String,
    pub canonical_headers: String,
    pub signed_headers: String,
    pub payload: String,
}

impl From<&generate_signed_url::RequestBody> for CanoncialRequest {
    fn from(value: &generate_signed_url::RequestBody) -> Self {
        let datestamp = value.now.format("%Y%m%d").to_string();

        let credential = format!(
            "{}/{datestamp}/auto/storage/goog4_request",
            value.service_account_email
        );
        let encoded_credentials = urlencoding::encode(&credential);

        let iso_date = value.now.format("%Y%m%dT%H%M%SZ").to_string();

        let canonical_querystring = format!(
            "X-Goog-Algorithm={SIGNING_ALGORITHM}&X-Goog-Credential={encoded_credentials}&X-Goog-Date={iso_date}&X-Goog-Expires={}&X-Goog-SignedHeaders=host",
            value.expires_secs,
        );
        let canonical_headers = "host:storage.googleapis.com";
        let signed_headers = "host";
        let payload_hash = "UNSIGNED-PAYLOAD";

        // Canonical request
        Self {
            http_verb: value.method.clone(),
            canonical_uri: value.object_path.clone(),
            canonical_querystring,
            canonical_headers: canonical_headers.to_string(),
            signed_headers: signed_headers.to_string(),
            payload: payload_hash.to_string(),
        }
    }
}

impl std::fmt::Display for CanoncialRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            http_verb,
            canonical_uri,
            canonical_querystring,
            canonical_headers,
            signed_headers,
            payload,
        } = self;

        write!(
            f,
            r#"{http_verb}
{canonical_uri}
{canonical_querystring}
{canonical_headers}

{signed_headers}
{payload}"#
        )
    }
}
