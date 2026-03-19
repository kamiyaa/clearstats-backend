use axum::Router;
use axum::body::Body;
use axum::http::{
    Method, Request,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use axum::response::Response;
use serde::Serialize;
use tower::ServiceExt;

#[derive(Clone, Debug)]
pub struct JsonRequestParams<T> {
    pub app: Router,
    pub method: Method,
    pub api_url: String,
    pub jwt: Option<String>,
    pub body: T,
}

pub async fn send_json_request<T: Serialize>(params: JsonRequestParams<T>) -> Response<Body> {
    let JsonRequestParams {
        app,
        method,
        api_url,
        jwt,
        body,
    } = params;

    let mut request_builder = Request::builder().uri(api_url).method(method);

    if let Some(jwt) = jwt {
        request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {jwt}"));
    }
    let request = request_builder
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&body).expect("Failed to serialize RequestBody"))
        .expect("Failed to build request");
    app.oneshot(request).await.expect("Failed to send request")
}

#[derive(Clone, Debug)]
pub struct EmptyRequestParams {
    pub app: Router,
    pub method: Method,
    pub api_url: String,
    pub jwt: Option<String>,
}
pub async fn send_empty_request(params: EmptyRequestParams) -> Response<Body> {
    let EmptyRequestParams {
        app,
        method,
        api_url,
        jwt,
    } = params;

    let mut request_builder = Request::builder().uri(api_url).method(method);

    if let Some(jwt) = jwt {
        request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {jwt}"));
    }
    let request = request_builder
        .body(Body::empty())
        .expect("Failed to build request");
    app.oneshot(request).await.expect("Failed to send request")
}
