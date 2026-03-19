use std::time::Duration;

use reqwest::Method;

use crate::utils::request::DEFAULT_TIMEOUT;

pub fn create_authenticated_request(
    api_key: &str,
    method: Method,
    url: &str,
) -> reqwest::RequestBuilder {
    let bearer = format!("Bearer {}", &api_key);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.append(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).expect("Failed to serialize bearer"),
    );
    reqwest::Client::new()
        .request(method, url)
        .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
        .headers(headers)
}
