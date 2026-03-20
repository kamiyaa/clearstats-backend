use std::collections::HashMap;

use axum::http::StatusCode;

use shared_lib::database::manager::DatabaseManager;
use shared_lib::error::{AppServerResult, ServerErrorResponse};

use crate::database::{authors, statistics};
use crate::types::{
    AttachmentResponse, AuthorResponse, SourceResponse, StatisticResponse, UserResponse,
};
use crate::utils::time::unix_secs_to_iso;

pub struct StatisticRow {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub user_vote: Option<i8>,
    pub question_count: u64,
    pub created_at: u64,
    pub updated_at: u64,
    pub posted_by_id: u64,
    pub posted_by_username: String,
    pub posted_by_created_at: u64,
}

pub async fn build_statistic_responses(
    db_manager: &DatabaseManager,
    rows: Vec<StatisticRow>,
) -> AppServerResult<Vec<StatisticResponse>> {
    if rows.is_empty() {
        return Ok(vec![]);
    }
    let statistic_ids: Vec<u64> = rows.iter().map(|r| r.id).collect();

    let tags = statistics::fetch_tags_by_statistic_ids::run_query(db_manager, &statistic_ids)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch tags");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch tags".to_string(),
            )
        })?;

    let sources = statistics::fetch_sources_by_statistic_ids::run_query(db_manager, &statistic_ids)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch sources");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch sources".to_string(),
            )
        })?;

    let attachments =
        statistics::fetch_attachments_by_statistic_ids::run_query(db_manager, &statistic_ids)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to fetch attachments");
                ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1234,
                    "Failed to fetch attachments".to_string(),
                )
            })?;

    let author_id_rows =
        statistics::fetch_author_ids_by_statistic_ids::run_query(db_manager, &statistic_ids)
            .await
            .map_err(|err| {
                tracing::error!(?err, "Failed to fetch author ids");
                ServerErrorResponse::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    1234,
                    "Failed to fetch author ids".to_string(),
                )
            })?;

    let all_author_ids: Vec<u64> = {
        let mut ids: Vec<u64> = author_id_rows.iter().map(|r| r.author_id).collect();
        ids.sort();
        ids.dedup();
        ids
    };

    let author_rows = authors::fetch_authors_by_ids::run_query(db_manager, &all_author_ids)
        .await
        .map_err(|err| {
            tracing::error!(?err, "Failed to fetch authors");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                "Failed to fetch authors".to_string(),
            )
        })?;

    // Build lookup maps
    let mut tags_by_id: HashMap<u64, Vec<String>> = HashMap::new();
    for t in tags {
        tags_by_id.entry(t.statistic_id).or_default().push(t.tag);
    }

    let mut sources_by_id: HashMap<u64, Vec<SourceResponse>> = HashMap::new();
    for s in sources {
        sources_by_id
            .entry(s.statistic_id)
            .or_default()
            .push(SourceResponse {
                id: s.id,
                url: s.url,
                title: s.title,
            });
    }

    let mut attachments_by_id: HashMap<u64, Vec<AttachmentResponse>> = HashMap::new();
    for a in attachments {
        attachments_by_id
            .entry(a.statistic_id)
            .or_default()
            .push(AttachmentResponse {
                id: a.id,
                url: a.url,
                filename: a.filename,
            });
    }

    let authors_map: HashMap<u64, AuthorResponse> = author_rows
        .into_iter()
        .map(|a| {
            (
                a.id,
                AuthorResponse {
                    id: a.id,
                    name: a.name,
                    bio: a.bio,
                    avatar_url: a.avatar_url,
                    affiliation: a.affiliation,
                },
            )
        })
        .collect();

    let mut author_ids_by_statistic: HashMap<u64, Vec<u64>> = HashMap::new();
    for r in author_id_rows {
        author_ids_by_statistic
            .entry(r.statistic_id)
            .or_default()
            .push(r.author_id);
    }

    let result = rows
        .into_iter()
        .map(|row| {
            let authors = author_ids_by_statistic
                .get(&row.id)
                .map(|ids| {
                    ids.iter()
                        .filter_map(|id| authors_map.get(id).cloned())
                        .collect()
                })
                .unwrap_or_default();

            StatisticResponse {
                id: row.id,
                title: row.title,
                description: row.description,
                tags: tags_by_id.remove(&row.id).unwrap_or_default(),
                sources: sources_by_id.remove(&row.id).unwrap_or_default(),
                attachments: attachments_by_id.remove(&row.id).unwrap_or_default(),
                authors,
                posted_by: UserResponse {
                    id: row.posted_by_id,
                    username: row.posted_by_username,
                    created_at: unix_secs_to_iso(row.posted_by_created_at),
                },
                upvotes: row.upvotes,
                downvotes: row.downvotes,
                user_vote: row.user_vote,
                question_count: row.question_count,
                created_at: unix_secs_to_iso(row.created_at),
                updated_at: unix_secs_to_iso(row.updated_at),
            }
        })
        .collect();

    Ok(result)
}
