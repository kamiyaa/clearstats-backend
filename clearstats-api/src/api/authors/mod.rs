pub mod get;
pub mod search;
pub mod statistics;

use axum::routing::get;
use shared_lib::server::cors::all_origin_cors;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route("/authors/search", get(search::handler))
        .route("/authors/{id}", get(get::handler))
        .route("/authors/{id}/statistics", get(statistics::handler))
        .layer(all_origin_cors())
}
