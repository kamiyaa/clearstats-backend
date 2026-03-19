pub mod create;
pub mod get;
pub mod list;
pub mod vote;

use axum::routing::{delete, get, post};
use shared_lib::server::cors::all_origin_cors;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route("/statistics", get(list::handler))
        .route("/statistics", post(create::handler))
        .route("/statistics/{id}", get(get::handler))
        .route("/statistics/{id}/vote", post(vote::upsert_handler))
        .route("/statistics/{id}/vote", delete(vote::delete_handler))
        .layer(all_origin_cors())
}
