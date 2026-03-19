pub mod reset_verification_code;
pub mod send_verification_email;
pub mod verify_email;

use axum::routing::post;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route(
            "/email/send_verification_email",
            post(send_verification_email::handler),
        )
        .route(
            "/email/reset_verification_code",
            post(reset_verification_code::handler),
        )
        .route("/email/verify_email", post(verify_email::handler))
}
