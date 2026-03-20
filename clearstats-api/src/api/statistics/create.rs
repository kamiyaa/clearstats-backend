use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use serde::Deserialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;
use shared_lib::utils::time::get_secs_since_epoch;

use crate::AppState;
use crate::database::statistics::{fetch_statistic_by_id, insert_statistic};
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::StatisticResponse;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub sources: Vec<SourceInput>,
    pub attachments: Vec<AttachmentInput>,
    pub author_ids: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct SourceInput {
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AttachmentInput {
    pub url: String,
    pub filename: String,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<RequestBody>,
) -> AppServerResult<ServerSuccessResponse<StatisticResponse>> {
    let token =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())?;

    let created_at = get_secs_since_epoch()?;
    let sources: Vec<(String, Option<String>)> = payload
        .sources
        .into_iter()
        .map(|s| (s.url, s.title))
        .collect();
    let attachments: Vec<(String, String)> = payload
        .attachments
        .into_iter()
        .map(|a| (a.url, a.filename))
        .collect();

    let db_manager = app_state.get_db_manager();

    let sql_data = insert_statistic::SqlData {
        title: &payload.title,
        description: &payload.description,
        posted_by_user_id: token.user.user_id,
        tags: &payload.tags,
        sources: &sources,
        attachments: &attachments,
        author_ids: &payload.author_ids,
        created_at,
    };

    let statistic_id = insert_statistic::run_query(db_manager, &sql_data)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to create statistic");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to create statistic".to_string(),
            )
        })?;

    let row = fetch_statistic_by_id::run_query(db_manager, statistic_id, Some(token.user.user_id))
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch created statistic");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch statistic".to_string(),
            )
        })?
        .ok_or_else(|| {
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Statistic not found after insert".to_string(),
            )
        })?;

    let rows = vec![StatisticRow {
        id: row.id,
        title: row.title,
        description: row.description,
        upvotes: row.upvotes,
        downvotes: row.downvotes,
        user_vote: row.user_vote,
        question_count: row.question_count,
        created_at: row.created_at,
        updated_at: row.updated_at,
        posted_by_id: row.posted_by_id,
        posted_by_username: row.posted_by_username,
        posted_by_created_at: row.posted_by_created_at,
    }];

    let mut stats = build_statistic_responses(db_manager, rows).await?;
    let stat = stats.remove(0);

    Ok(ServerSuccessResponse::new_with_status(
        axum::http::StatusCode::CREATED,
        stat,
    ))
}
