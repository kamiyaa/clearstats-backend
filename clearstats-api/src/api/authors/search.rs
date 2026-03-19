use axum::extract::{Query, State};
use axum::http::StatusCode;
use serde::Deserialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};

use crate::AppState;
use crate::database::authors::search_authors;
use crate::types::AuthorResponse;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub q: Option<String>,
}

pub async fn handler(
    State(app_state): State<AppState>,
    Query(params): Query<QueryParams>,
) -> AppServerResult<ServerSuccessResponse<Vec<AuthorResponse>>> {
    let q = params.q.unwrap_or_default();
    let db_manager = app_state.get_db_manager();

    let authors = search_authors::run_query(db_manager, q.trim())
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to search authors");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to search authors".to_string(),
            )
        })?;

    let result = authors
        .into_iter()
        .map(|a| AuthorResponse {
            id: a.id,
            name: a.name,
            bio: a.bio,
            avatar_url: a.avatar_url,
            affiliation: a.affiliation,
        })
        .collect();

    Ok(ServerSuccessResponse::new(result))
}
