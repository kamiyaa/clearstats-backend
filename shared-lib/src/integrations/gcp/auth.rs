use std::{
    ops::Add,
    time::{Duration, SystemTime},
};

use axum::http::Method;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::error::AppServerResult;
use crate::{
    integrations::gcp::CLOUD_RUN_AUTH_SERVER,
    utils::{
        request::{DEFAULT_TIMEOUT, reqwest_send_to_server_error},
        response::try_parse_json_response,
    },
};

#[derive(Clone, Debug)]
pub struct ServiceAccountAuthToken {
    pub access_token: String,
    pub expires_at: SystemTime,
    pub token_type: String,
}

impl std::convert::From<ServiceAccountAuthTokenRaw> for ServiceAccountAuthToken {
    fn from(value: ServiceAccountAuthTokenRaw) -> Self {
        let now = SystemTime::now();
        Self {
            access_token: value.access_token,
            expires_at: now.add(Duration::from_secs(value.expires_in as u64)),
            token_type: value.token_type,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServiceAccountAuthTokenRaw {
    pub access_token: String,
    pub expires_in: usize,
    pub token_type: String,
}

pub async fn fetch_service_account_auth_token() -> AppServerResult<ServiceAccountAuthTokenRaw> {
    // FOR DEVELOPMENT PURPOSES ONLY
    // Devs will export this variable via gcloud to interface
    // and authenticate with GCP
    if let Ok(value) = std::env::var("GCP_ACCESS_TOKEN") {
        let service_token = ServiceAccountAuthTokenRaw {
            access_token: value,
            expires_in: 3000,
            token_type: "access_token".to_string(),
        };
        return Ok(service_token);
    }

    // regular flow
    let api_url = CLOUD_RUN_AUTH_SERVER;
    let resp = reqwest::Client::new()
        .request(reqwest::Method::GET, api_url)
        .header(
            "Metadata-Flavor",
            reqwest::header::HeaderValue::from_static("Google"),
        )
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
        .send()
        .await
        .map_err(reqwest_send_to_server_error)?;
    try_parse_json_response(resp).await
}

pub fn create_authenticated_request(
    access_token: &str,
    method: Method,
    url: &str,
) -> reqwest::RequestBuilder {
    let url = reqwest::Url::parse(url).expect("Failed to parse url");

    let bearer = format!("Bearer {}", access_token);
    let mut headers = HeaderMap::new();
    headers.append(
        "Authorization",
        HeaderValue::from_str(&bearer).expect("Failed to serialize bearer"),
    );
    reqwest::Client::new()
        .request(method, url)
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
        .headers(headers)
}
