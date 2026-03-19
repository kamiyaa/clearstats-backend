use axum::http::StatusCode;

use crate::error::ServerErrorResponse;

pub const DEFAULT_TIMEOUT: u64 = 30;

pub fn reqwest_send_to_server_error(err: reqwest::Error) -> ServerErrorResponse {
    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1234, err.to_string())
}

pub fn reqwest_text_to_server_error(err: reqwest::Error) -> ServerErrorResponse {
    let error_msg = "Failed to parse request body";
    tracing::error!(?err, "{error_msg}");
    ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1234, err.to_string())
}
