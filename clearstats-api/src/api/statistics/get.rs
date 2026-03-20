use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::statistics::fetch_statistic_by_id;
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::StatisticResponse;

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<u64>,
) -> AppServerResult<ServerSuccessResponse<StatisticResponse>> {
    let current_user_id =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())
            .ok()
            .map(|t| t.user.user_id);

    let db_manager = app_state.get_db_manager();

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

    let mut stats = build_statistic_responses(db_manager, rows, current_user_id).await?;
    let stat = stats.remove(0);

    Ok(ServerSuccessResponse::new(stat))
}
