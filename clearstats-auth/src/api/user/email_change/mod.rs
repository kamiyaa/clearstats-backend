pub mod cancel_email_change;
pub mod fetch_pending_email;
pub mod send_email_change_verification;
pub mod verify_email_change;
use axum::routing::{delete, get, post};

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .route(
            "/email/change_request",
            post(send_email_change_verification::handler),
        )
        .route(
            "/email/verify_change_request",
            post(verify_email_change::handler),
        )
        .route("/email/change_request", get(fetch_pending_email::handler))
        .route(
            "/email/change_request",
            delete(cancel_email_change::handler),
        )
    //.route("/email_change/pending_change", get(verify_email::handler))
}
