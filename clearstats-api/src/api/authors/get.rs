use axum::extract::{Path, State};
use axum::http::StatusCode;

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};

use crate::AppState;
use crate::database::authors::fetch_author_by_id;
use crate::types::AuthorResponse;

pub async fn handler(
    State(app_state): State<AppState>,
    Path(id): Path<DatabaseInteger>,
) -> AppServerResult<ServerSuccessResponse<AuthorResponse>> {
    let db_manager = app_state.get_db_manager();

    let author = fetch_author_by_id::run_query(db_manager, id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch author");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch author".to_string(),
            )
        })?
        .ok_or_else(|| {
            ServerErrorResponse::new(StatusCode::NOT_FOUND, 1234, "Author not found".to_string())
        })?;

    Ok(ServerSuccessResponse::new(AuthorResponse {
        id: author.id,
        name: author.name,
        bio: author.bio,
        avatar_url: author.avatar_url,
        affiliation: author.affiliation,
    }))
}
