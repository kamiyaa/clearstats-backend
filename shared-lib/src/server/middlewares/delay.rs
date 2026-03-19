use axum::{body, http::Request, middleware::Next, response::Response};

pub async fn intentional_delay(request: Request<body::Body>, next: Next) -> Response {
    // used for testing
    std::thread::sleep(std::time::Duration::from_secs(1));
    next.run(request).await
}
