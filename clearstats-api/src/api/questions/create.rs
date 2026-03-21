use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use serde::Deserialize;

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;
use shared_lib::utils::time::get_secs_since_epoch;

use crate::AppState;
use crate::database::questions::insert_question;
use crate::types::{QuestionResponse, UserResponse};
use crate::utils::time::unix_secs_to_iso;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub body: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(statistic_id): Path<DatabaseInteger>,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<QuestionResponse>> {
    let token =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())?;

    if payload.body.trim().is_empty() {
        return Err(ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            "Question body cannot be empty".to_string(),
        ));
    }

    let created_at = get_secs_since_epoch()?;
    let db_manager = app_state.get_db_manager();

    let sql_data = insert_question::SqlData {
        statistic_id,
        body: payload.body.trim(),
        posted_by_user_id: token.user.user_id,
        created_at,
    };

    let row = insert_question::run_query(db_manager, &sql_data)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to create question");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to create question".to_string(),
            )
        })?;

    let question = QuestionResponse {
        id: row.id,
        statistic_id: row.statistic_id,
        body: row.body,
        posted_by: UserResponse {
            id: row.posted_by_id,
            username: row.posted_by_username,
            created_at: unix_secs_to_iso(row.posted_by_created_at),
        },
        upvotes: row.upvotes,
        downvotes: row.downvotes,
        user_vote: None,
        created_at: unix_secs_to_iso(row.created_at),
    };

    Ok(ServerSuccessResponse::new_with_status(
        axum::http::StatusCode::CREATED,
        question,
    ))
}
