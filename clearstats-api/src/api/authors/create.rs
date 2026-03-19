use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use serde::Deserialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::authors::insert_author;
use crate::types::AuthorResponse;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub affiliation: Option<String>,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<AuthorResponse>> {
    let _token =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())?;

    if payload.name.trim().is_empty() {
        return Err(ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            "Author name cannot be empty".to_string(),
        ));
    }

    let db_manager = app_state.get_db_manager();

    let sql_data = insert_author::SqlData {
        name: payload.name.trim(),
        bio: payload.bio.as_deref(),
        avatar_url: payload.avatar_url.as_deref(),
        affiliation: payload.affiliation.as_deref(),
    };

    let row = insert_author::run_query(db_manager, &sql_data)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to create author");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to create author".to_string(),
            )
        })?;

    let author = AuthorResponse {
        id: row.id,
        name: row.name,
        bio: row.bio,
        avatar_url: row.avatar_url,
        affiliation: row.affiliation,
    };

    Ok(ServerSuccessResponse::new_with_status(
        StatusCode::CREATED,
        author,
    ))
}
