use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::error::{AppServerResult, ServerErrorResponse};

/// Parses response body to a string
pub async fn parse_string_response(resp: Response) -> AppServerResult<String> {
    let resp_text = resp.text().await.map_err(|err| {
        let error_msg = "Failed to parse body";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
    })?;
    Ok(resp_text)
}

/// Parses response body to a json
pub async fn parse_json_response<T: DeserializeOwned>(resp: Response) -> AppServerResult<T> {
    let resp_string = parse_string_response(resp).await?;
    let resp_json = serde_json::from_str(&resp_string).map_err(|err| {
        let error_msg = "Failed to parse JSON body";
        tracing::error!(?err, resp_string, "{error_msg}");
        ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
    })?;
    Ok(resp_json)
}

pub async fn try_parse_json_response<T: DeserializeOwned>(resp: Response) -> AppServerResult<T> {
    let status_code = resp.status();
    if !status_code.is_success() {
        let body = resp.text().await.map_err(|err| {
            tracing::error!(?err, "Failed to parse error response body");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, err.to_string())
        })?;
        let err = ServerErrorResponse::new(status_code, 1000, body);
        return Err(err);
    }
    let resp = parse_json_response(resp).await?;

    Ok(resp)
}
