use axum::extract::State;
use axum_extra::extract::CookieJar;
use shared_lib::error::AppServerResult;

use crate::{AppState, utils::jwt::generate_refresh_token_cookie};

pub async fn handler(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> AppServerResult<CookieJar> {
    let mut refresh_cookie = generate_refresh_token_cookie(String::new(), &app_state.config);
    let max_age = cookie::time::Duration::new(-1, 0);
    refresh_cookie.set_max_age(max_age);
    Ok(jar.add(refresh_cookie))
}
