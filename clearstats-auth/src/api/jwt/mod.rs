pub mod gen_access_token;
pub mod gen_refresh_token;

use axum::routing::post;
use shared_lib::server::cors::credentials_cors;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route("/refresh/access_token", post(gen_access_token::handler))
        .route("/refresh/refresh_token", post(gen_refresh_token::handler))
        .layer(credentials_cors())
}
