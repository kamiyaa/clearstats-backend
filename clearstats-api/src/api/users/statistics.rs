use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::statistics::{fetch_statistic_by_id, fetch_statistic_ids_by_user_id};
use crate::database::users::fetch_user_by_username;
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::StatisticResponse;

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(username): Path<String>,
) -> AppServerResult<ServerSuccessResponse<Vec<StatisticResponse>>> {
    let current_user_id =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())
            .ok()
            .map(|t| t.user.user_id);

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

    let id_rows = fetch_statistic_ids_by_user_id::run_query(db_manager, user.id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch statistic ids");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch statistics".to_string(),
            )
        })?;

    let mut rows = Vec::new();
    for id_row in id_rows {
        if let Some(row) = fetch_statistic_by_id::run_query(db_manager, id_row.id)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to fetch statistic");
                ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1234,
                    "Failed to fetch statistic".to_string(),
                )
            })?
        {
            rows.push(StatisticRow {
                id: row.id,
                title: row.title,
                description: row.description,
                upvotes: row.upvotes,
                downvotes: row.downvotes,
                question_count: row.question_count,
                created_at: row.created_at,
                posted_by_id: row.posted_by_id,
                posted_by_username: row.posted_by_username,
                posted_by_email: row.posted_by_email,
                posted_by_created_at: row.posted_by_created_at,
            });
        }
    }

    let stats = build_statistic_responses(db_manager, rows, current_user_id).await?;
    Ok(ServerSuccessResponse::new(stats))
}
