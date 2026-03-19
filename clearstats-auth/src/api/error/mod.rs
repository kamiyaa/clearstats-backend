pub mod test_error;

use axum::routing::get;
use shared_lib::server::cors::all_origin_cors;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route("/test_error", get(test_error::handler))
        .layer(all_origin_cors())
}
