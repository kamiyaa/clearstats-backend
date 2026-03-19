pub mod error;
pub mod jwt;
pub mod user;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .merge(jwt::router())
        .merge(user::router())
        .merge(error::router())
}
