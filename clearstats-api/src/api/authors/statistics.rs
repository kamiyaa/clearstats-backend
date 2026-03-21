use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::statistics::fetch_statistic_by_id;
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::StatisticResponse;

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(author_id): Path<DatabaseInteger>,
) -> AppServerResult<ServerSuccessResponse<Vec<StatisticResponse>>> {
    use shared_lib::database::manager::DatabaseManagerTrait;

    let current_user_id =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())
            .ok()
            .map(|t| t.user.user_id);

    let db_manager = app_state.get_db_manager();

    // Get statistic IDs for this author
    let id_rows: Vec<(DatabaseInteger,)> = sqlx::query_as(
        "SELECT statistic_id FROM statistic_author WHERE author_id = $1 ORDER BY statistic_id DESC",
    )
    .bind(author_id)
    .fetch_all(db_manager.get_database_pool())
    .await
    .map_err(|err| {
        tracing::error!(?err, "Failed to fetch author statistic ids");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            "Failed to fetch statistics".to_string(),
        )
    })?;

    let mut rows = Vec::new();
    for (id,) in id_rows {
        if let Some(row) = fetch_statistic_by_id::run_query(db_manager, id, current_user_id)
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
                user_vote: row.user_vote,
                question_count: row.question_count,
                created_at: row.created_at,
                updated_at: row.updated_at,
                posted_by_id: row.posted_by_id,
                posted_by_username: row.posted_by_username,
                posted_by_created_at: row.posted_by_created_at,
            });
        }
    }

    let stats = build_statistic_responses(db_manager, rows).await?;
    Ok(ServerSuccessResponse::new(stats))
}
