use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::database::statistics::fetch_statistics;
use crate::helpers::{StatisticRow, build_statistic_responses};
use crate::types::StatisticResponse;
use crate::{AppState, PAGE_SIZE};

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub page: Option<DatabaseInteger>,
    pub search: Option<String>,
    pub tag: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ResponseBody {
    pub items: Vec<StatisticResponse>,
}

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Query(query_params): Query<QueryParams>,
) -> AppServerResult<ServerSuccessResponse<ResponseBody>> {
    let QueryParams { page, search, tag } = query_params;

    let page_index = page.unwrap_or(0);
    let page_size = PAGE_SIZE;

    let current_user_id =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())
            .ok()
            .map(|t| t.user.user_id);

    let db_manager = app_state.get_db_manager();

    let sql_query = fetch_statistics::SqlQuery {
        user_id: current_user_id,
        search,
        tag,
        page_size,
        page_index,
    };

    tracing::debug!(?sql_query, "Fetching statistics");
    let rows = fetch_statistics::run_query(db_manager, &sql_query)
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
            user_vote: r.user_vote,
            question_count: r.question_count,
            created_at: r.created_at,
            updated_at: r.updated_at,
            posted_by_id: r.posted_by_id,
            posted_by_username: r.posted_by_username,
            posted_by_created_at: r.posted_by_created_at,
        })
        .collect();
    let items = build_statistic_responses(db_manager, rows).await?;

    let resp = ResponseBody { items };
    Ok(ServerSuccessResponse::new(resp))
}
