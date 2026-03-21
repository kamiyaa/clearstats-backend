use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::statistics::{delete_vote, fetch_statistic_metrics_by_id, upsert_vote};

#[derive(Clone, Debug, Deserialize)]
pub struct RequestBody {
    pub vote: i8,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {
    pub id: DatabaseInteger,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub question_count: DatabaseInteger,
}

pub async fn upsert_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<DatabaseInteger>,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
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

    let row = fetch_statistic_metrics_by_id::run_query(db_manager, id)
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

    let resp = ResponseBody {
        id: row.id,
        upvotes: row.upvotes,
        downvotes: row.downvotes,
        question_count: row.question_count,
    };
    Ok(ServerSuccessResponse::new(resp))
}

pub async fn delete_handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<DatabaseInteger>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
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


    let row = fetch_statistic_metrics_by_id::run_query(db_manager, id)
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

    let resp = ResponseBody {
        id: row.id,
        upvotes: row.upvotes,
        downvotes: row.downvotes,
        question_count: row.question_count,
    };
    Ok(ServerSuccessResponse::new(resp))
}
