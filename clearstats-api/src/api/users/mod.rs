pub mod profile;
pub mod statistics;

use axum::routing::get;
use shared_lib::server::cors::all_origin_cors;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route("/users/{username}", get(profile::handler))
        .route("/users/{username}/statistics", get(statistics::handler))
        .layer(all_origin_cors())
}
