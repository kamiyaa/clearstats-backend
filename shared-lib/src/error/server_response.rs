use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct ServerSuccessResponse<T: Serialize> {
    pub status_code: StatusCode,
    pub body: ServerSuccessBody<T>,
}

impl<T: Serialize> ServerSuccessResponse<T> {
    pub fn new(result: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            body: ServerSuccessBody { ok: true, result },
        }
    }

    pub fn new_with_status(status_code: StatusCode, result: T) -> Self {
        Self {
            status_code,
            body: ServerSuccessBody { ok: true, result },
        }
    }
}

impl<T: Serialize> IntoResponse for ServerSuccessResponse<T> {
    fn into_response(self) -> Response<body::Body> {
        let json_body = serde_json::to_string(&self.body).unwrap_or("{}".to_owned());
        let response = Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .status(self.status_code)
            .body(body::Body::new(json_body));
        response.expect("Failed to build response")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSuccessBody<T: Serialize> {
    pub ok: bool,
    pub result: T,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerErrorBody {
    pub ok: bool,
    pub result: ServerErrorBodyInner,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerErrorBodyInner {
    pub error_code: usize,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct ServerErrorResponse {
    pub status_code: StatusCode,
    pub body: ServerErrorBody,
}
impl ServerErrorResponse {
    pub fn new(status_code: StatusCode, error_code: usize, message: String) -> Self {
        Self {
            status_code,
            body: ServerErrorBody {
                ok: false,
                result: ServerErrorBodyInner {
                    error_code,
                    message,
                },
            },
        }
    }
}

impl std::fmt::Display for ServerErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.body.result.message)
    }
}

impl IntoResponse for ServerErrorResponse {
    fn into_response(self) -> Response<body::Body> {
        let json_body = serde_json::to_string(&self.body).unwrap_or("{}".to_owned());
        let response = Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .status(self.status_code)
            .body(body::Body::new(json_body));
        response.expect("Failed to build response")
    }
}
