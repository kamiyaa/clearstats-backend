use std::time::Instant;

use axum::{body, http::Request, middleware::Next, response::Response};
use chrono::Utc;
use colored::*;

pub const LOGGER_TARGET: &str = "server_response";

pub async fn structured_logger(request: Request<body::Body>, next: Next) -> Response {
    let request_method = request.method().to_string();
    let request_url = request.uri().to_string();

    let now = Instant::now();
    let response = next.run(request).await;

    let response_status = response.status();
    if response_status.is_success() {
        tracing::info!(target: LOGGER_TARGET, method=request_method,
            status_code=response_status.as_u16(),
            elapsed=?now.elapsed(),
            url=request_url, "Response");
    } else if response_status.is_server_error() {
        tracing::error!(target: LOGGER_TARGET, method=request_method,
            status_code=response_status.as_u16(),
            elapsed=?now.elapsed(),
            url=request_url, "Response");
    } else {
        tracing::warn!(target: LOGGER_TARGET, method=request_method,
            status_code=response_status.as_u16(),
            elapsed=?now.elapsed(),
            url=request_url, "Response");
    }
    response
}

pub async fn terminal_logger(request: Request<body::Body>, next: Next) -> Response {
    let request_timestamp = Utc::now();
    let request_method = request.method().to_string();
    let request_url = request.uri().to_string();

    println!(
        "{request_msg}
    {request_method} {request_url}
    timestamp {request_timestamp}
",
        request_msg = "Request received".bold().green(),
    );
    let response = next.run(request).await;

    let response_timestamp = Utc::now();
    let response_status = response.status();
    let response_status = if response_status.is_success() {
        response_status.to_string().green()
    } else if response_status.is_server_error() {
        response_status.to_string().red()
    } else {
        response_status.to_string().yellow()
    };

    println!(
        "{response_msg}
    {request_method} {request_url}
    status {response_status}
    timestamp {response_timestamp}
",
        response_msg = "Response sent".bold().green(),
    );
    response
}
