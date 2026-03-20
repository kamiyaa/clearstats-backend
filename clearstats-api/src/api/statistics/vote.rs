use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use serde::Deserialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::statistics::{delete_vote, fetch_statistic_by_id, upsert_vote};
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::StatisticResponse;

#[derive(Debug, Deserialize)]
pub struct VoteBody {
    pub vote: i8,
}

pub async fn upsert_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<u64>,
    Json(payload): Json<VoteBody>,
) -> AppServerResult<ServerSuccessResponse<StatisticResponse>> {
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

    upsert_vote::run_query(db_manager, id, token.user.user_id, payload.vote)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to upsert vote");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to vote".to_string(),
            )
        })?;

    let row = fetch_statistic_by_id::run_query(db_manager, id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch statistic");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch statistic".to_string(),
            )
        })?
        .ok_or_else(|| {
            ServerErrorResponse::new(
                StatusCode::NOT_FOUND,
                1234,
                "Statistic not found".to_string(),
            )
        })?;

    let rows = vec![StatisticRow {
        id: row.id,
        title: row.title,
        description: row.description,
        upvotes: row.upvotes,
        downvotes: row.downvotes,
        question_count: row.question_count,
        created_at: row.created_at,
        updated_at: row.updated_at,
        posted_by_id: row.posted_by_id,
        posted_by_username: row.posted_by_username,
        posted_by_created_at: row.posted_by_created_at,
    }];

    let mut stats = build_statistic_responses(db_manager, rows, Some(token.user.user_id)).await?;
    Ok(ServerSuccessResponse::new(stats.remove(0)))
}

pub async fn delete_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<u64>,
) -> AppServerResult<ServerSuccessResponse<StatisticResponse>> {
    let token =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())?;

    let db_manager = app_state.get_db_manager();

    delete_vote::run_query(db_manager, id, token.user.user_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to delete vote");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to remove vote".to_string(),
            )
        })?;

    let row = fetch_statistic_by_id::run_query(db_manager, id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch statistic");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch statistic".to_string(),
            )
        })?
        .ok_or_else(|| {
            ServerErrorResponse::new(
                StatusCode::NOT_FOUND,
                1234,
                "Statistic not found".to_string(),
            )
        })?;

    let rows = vec![StatisticRow {
        id: row.id,
        title: row.title,
        description: row.description,
        upvotes: row.upvotes,
        downvotes: row.downvotes,
        question_count: row.question_count,
        created_at: row.created_at,
        updated_at: row.updated_at,
        posted_by_id: row.posted_by_id,
        posted_by_username: row.posted_by_username,
        posted_by_created_at: row.posted_by_created_at,
    }];

    let mut stats = build_statistic_responses(db_manager, rows, Some(token.user.user_id)).await?;
    Ok(ServerSuccessResponse::new(stats.remove(0)))
}
