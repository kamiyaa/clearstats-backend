pub mod change_password;
pub mod email_change;
pub mod get_email;
pub mod login;
pub mod logout;
pub mod recovery;
pub mod register;
pub mod update_user;
pub mod verification;

use axum::routing::{get, post};
use shared_lib::server::cors::{all_origin_cors, credentials_cors};

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .merge(recovery::router())
        .merge(verification::router())
        .merge(email_change::router())
        .route("/user/update", post(update_user::handler))
        .route("/user/change_password", post(change_password::handler))
        .route("/email", get(get_email::handler))
        .layer(all_origin_cors())
        .merge(login_router())
}

fn login_router() -> ServerRouter {
    ServerRouter::new()
        .route("/user/register", post(register::handler))
        .route("/user/login", post(login::handler))
        .route("/user/logout", post(logout::handler))
        .layer(credentials_cors())
}
