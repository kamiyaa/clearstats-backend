use axum::http::{HeaderValue, Method, header};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};

pub const ALLOWED_ORIGINS: &[&str] = &[
    "http://localhost:5010",
    "http://localhost:5020",
    "https://lab.indaggo.com",
    "https://institution.indaggo.com",
    "https://dev-lab.indaggo.com",
    "https://dev-institution.indaggo.com",
];

pub fn allow_origin_list() -> AllowOrigin {
    AllowOrigin::list(
        ALLOWED_ORIGINS
            .iter()
            .map(|s| HeaderValue::from_static(s))
            .collect::<Vec<_>>(),
    )
}

/// Used by default for all services
pub fn all_origin_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(AllowHeaders::any())
        .allow_methods(AllowMethods::any())
}

/// used by auth service to allow setting and removing cookies
pub fn credentials_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_credentials(true)
        .allow_origin(AllowOrigin::list(
            ALLOWED_ORIGINS
                .iter()
                .map(|s| HeaderValue::from_static(s))
                .collect::<Vec<_>>(),
        ))
        .allow_headers([
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ACCEPT_ENCODING,
            header::ACCEPT_LANGUAGE,
            header::CONNECTION,
            header::CONTENT_ENCODING,
            header::CONTENT_LENGTH,
            header::CONTENT_RANGE,
            header::CONTENT_TYPE,
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ])
}
