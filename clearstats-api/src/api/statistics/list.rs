use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use serde::Deserialize;

use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::statistics::fetch_statistics;
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::PaginatedResponse;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub search: Option<String>,
    pub tag: Option<String>,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<QueryParams>,
) -> AppServerResult<ServerSuccessResponse<PaginatedResponse<crate::types::StatisticResponse>>> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let search = params.search.unwrap_or_default();
    let tag = params.tag.unwrap_or_default();
    let offset = (page - 1) * per_page;

    let current_user_id =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())
            .ok()
            .map(|t| t.user.user_id);

    let db_manager = app_state.get_db_manager();

    let total = fetch_statistics::count_query(db_manager, &search, &tag)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to count statistics");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to count statistics".to_string(),
            )
        })?;

    let rows = fetch_statistics::run_query(
        db_manager,
        &fetch_statistics::Params {
            search: &search,
            tag: &tag,
            limit: per_page,
            offset,
        },
    )
    .await
    .map_err(|err| {
        tracing::error!(?err, "Failed to fetch statistics");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            "Failed to fetch statistics".to_string(),
        )
    })?;

    let rows: Vec<StatisticRow> = rows
        .into_iter()
        .map(|r| StatisticRow {
            id: r.id,
            title: r.title,
            description: r.description,
            upvotes: r.upvotes,
            downvotes: r.downvotes,
            question_count: r.question_count,
            created_at: r.created_at,
            posted_by_id: r.posted_by_id,
            posted_by_username: r.posted_by_username,
            posted_by_email: r.posted_by_email,
            posted_by_created_at: r.posted_by_created_at,
        })
        .collect();

    let items = build_statistic_responses(db_manager, rows, current_user_id).await?;

    Ok(ServerSuccessResponse::new(PaginatedResponse {
        items,
        total,
        page,
        per_page,
    }))
}
