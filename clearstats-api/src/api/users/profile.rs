use axum::extract::{Path, State};
use axum::http::StatusCode;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};

use crate::AppState;
use crate::database::users::fetch_user_by_username;
use crate::types::UserResponse;
use crate::utils::time::unix_secs_to_iso;

pub async fn handler(
    State(app_state): State<AppState>,
    Path(username): Path<String>,
) -> AppServerResult<ServerSuccessResponse<UserResponse>> {
    let db_manager = app_state.get_db_manager();

    let user = fetch_user_by_username::run_query(db_manager, &username)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch user");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch user".to_string(),
            )
        })?
        .ok_or_else(|| {
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, "User not found".to_string())
        })?;

    Ok(ServerSuccessResponse::new(UserResponse {
        id: user.id,
        username: user.username,
        created_at: unix_secs_to_iso(user.created_at),
    }))
}
