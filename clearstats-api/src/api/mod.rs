pub mod authors;
pub mod questions;
pub mod statistics;
pub mod users;

use crate::ServerRouter;

pub fn router() -> ServerRouter {
    ServerRouter::new()
        .merge(statistics::router())
        .merge(questions::router())
        .merge(users::router())
        .merge(authors::router())
}
