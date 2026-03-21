use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};

use shared_lib::database::DatabaseInteger;
use shared_lib::error::{AppServerResult, ServerErrorResponse, ServerSuccessResponse};
use shared_lib::types::jwt::AccessToken;

use crate::AppState;
use crate::database::questions::{fetch_questions_by_statistic_id, fetch_votes_by_question_ids};
use crate::types::{QuestionResponse, UserResponse};
use crate::utils::time::unix_secs_to_iso;

pub async fn handler(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(statistic_id): Path<DatabaseInteger>,
) -> AppServerResult<ServerSuccessResponse<Vec<QuestionResponse>>> {
    let current_user_id =
        AccessToken::from_header_map_unverified(headers, app_state.config.get_jwt_token_secret())
            .ok()
            .map(|t| t.user.user_id);

    let db_manager = app_state.get_db_manager();

    let rows = fetch_questions_by_statistic_id::run_query(db_manager, statistic_id)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch questions");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch questions".to_string(),
            )
        })?;

    let question_ids: Vec<DatabaseInteger> = rows.iter().map(|r| r.id).collect();

    let votes: HashMap<DatabaseInteger, i8> = if let Some(user_id) = current_user_id {
        fetch_votes_by_question_ids::run_query(db_manager, &question_ids, user_id)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to fetch votes");
                ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1234,
                    "Failed to fetch votes".to_string(),
                )
            })?
            .into_iter()
            .map(|v| (v.question_id, v.vote))
            .collect()
    } else {
        HashMap::new()
    };

    let questions = rows
        .into_iter()
        .map(|r| QuestionResponse {
            id: r.id,
            statistic_id: r.statistic_id,
            body: r.body,
            posted_by: UserResponse {
                id: r.posted_by_id,
                username: r.posted_by_username,
                created_at: unix_secs_to_iso(r.posted_by_created_at),
            },
            upvotes: r.upvotes,
            downvotes: r.downvotes,
            user_vote: votes.get(&r.id).copied(),
            created_at: unix_secs_to_iso(r.created_at),
        })
        .collect();

    Ok(ServerSuccessResponse::new(questions))
}
