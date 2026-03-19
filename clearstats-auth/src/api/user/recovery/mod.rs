pub mod reset_password;
pub mod send_reset_password_email;

use axum::routing::post;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route(
            "/user/request_password_reset",
            post(send_reset_password_email::handler),
        )
        .route("/user/password_reset", post(reset_password::handler))
}
