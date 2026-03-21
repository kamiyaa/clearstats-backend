use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use serde::Deserialize;

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::questions::{
    delete_vote, fetch_questions_by_statistic_id, fetch_votes_by_question_ids, upsert_vote,
};
use crate::types::{QuestionResponse, UserResponse};
use crate::utils::time::unix_secs_to_iso;

#[derive(Debug, Deserialize)]
pub struct VoteBody {
    pub vote: i16,
}

pub async fn upsert_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path((statistic_id, question_id)): Path<(DatabaseInteger, DatabaseInteger)>,
    Json(payload): Json<VoteBody>,
) -> AppServerResult<ServerSuccessResponse<QuestionResponse>> {
    let token =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())?;

    if payload.vote != 1 && payload.vote != -1 {
        return Err(ServerErrorResponse::new(
            StatusCode::BAD_REQUEST,
            1234,
            "Vote must be 1 or -1".to_string(),
        ));
    }

    let db_manager = app_state.get_db_manager();

    upsert_vote::run_query(db_manager, question_id, token.user.user_id, payload.vote)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to upsert vote");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to vote".to_string(),
            )
        })?;

    fetch_question_response(db_manager, statistic_id, question_id, token.user.user_id).await
}

pub async fn delete_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path((statistic_id, question_id)): Path<(DatabaseInteger, DatabaseInteger)>,
) -> AppServerResult<ServerSuccessResponse<QuestionResponse>> {
    let token =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())?;

    let db_manager = app_state.get_db_manager();

    delete_vote::run_query(db_manager, question_id, token.user.user_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to delete vote");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to remove vote".to_string(),
            )
        })?;

    fetch_question_response(db_manager, statistic_id, question_id, token.user.user_id).await
}

async fn fetch_question_response(
    db_manager: &shared_lib::database::manager::DatabaseManager,
    statistic_id: DatabaseInteger,
    question_id: DatabaseInteger,
    user_id: DatabaseInteger,
) -> AppServerResult<ServerSuccessResponse<QuestionResponse>> {
    let rows = fetch_questions_by_statistic_id::run_query(db_manager, statistic_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch question");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch question".to_string(),
            )
        })?;

    let row = rows
        .into_iter()
        .find(|r| r.id == question_id)
        .ok_or_else(|| {
            ServerErrorResponse::new(
                StatusCode::NOT_FOUND,
                1234,
                "Question not found".to_string(),
            )
        })?;

    let votes = fetch_votes_by_question_ids::run_query(db_manager, &[question_id], user_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch votes");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch votes".to_string(),
            )
        })?;

    let user_vote = votes
        .into_iter()
        .find(|v| v.question_id == question_id)
        .map(|v| v.vote);

    Ok(ServerSuccessResponse::new(QuestionResponse {
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
        user_vote,
        created_at: unix_secs_to_iso(row.created_at),
    }))
}
