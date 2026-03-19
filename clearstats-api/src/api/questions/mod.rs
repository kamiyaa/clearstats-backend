pub mod create;
pub mod list;
pub mod vote;

use axum::routing::{delete, get, post};
use shared_lib::server::cors::all_origin_cors;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route("/statistics/{statistic_id}/questions", get(list::handler))
        .route(
            "/statistics/{statistic_id}/questions",
            post(create::handler),
        )
        .route(
            "/statistics/{statistic_id}/questions/{question_id}/vote",
            post(vote::upsert_handler),
        )
        .route(
            "/statistics/{statistic_id}/questions/{question_id}/vote",
            delete(vote::delete_handler),
        )
        .layer(all_origin_cors())
}
