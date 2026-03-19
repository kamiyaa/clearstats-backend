use crate::error::{AppServerResult, ServerErrorResponse};
use axum::body::Body;
use http_body_util::BodyExt;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

pub async fn body_to_json_response<T: DeserializeOwned>(body: Body) -> AppServerResult<T> {
    let resp_body: Vec<u8> = body
        .collect()
        .await
        .expect("Failed to collect body data")
        .to_bytes()
        .into_iter()
        .collect();

    let resp_body_string =
        String::from_utf8(resp_body).expect("Failed to parse body string to UTF-8");
    eprintln!("Response Body: {resp_body_string}");

    let result: T = serde_json::from_str(&resp_body_string).expect("Failed to parse body to JSON");

    Ok(result)
}

pub fn str_to_u64(s: &str) -> AppServerResult<u64> {
    s.parse::<u64>().map_err(|err| {
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
    })
}
