use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub question_count: u64,
    pub created_at: u64,
    pub posted_by_id: u64,
    pub posted_by_username: String,
    pub posted_by_email: String,
    pub posted_by_created_at: u64,
}

pub struct Params<'a> {
    pub search: &'a str,
    pub tag: &'a str,
    pub limit: u64,
    pub offset: u64,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    params: &Params<'_>,
) -> DatabaseResult<Vec<SqlData>> {
    let pool = db_manager.get_database_pool();

    if !params.tag.is_empty() {
        let search_pattern = format!("%{}%", params.search);
        let results = sqlx::query_as(
            "SELECT
                s.id,
                s.title,
                s.description,
                s.upvotes,
                s.downvotes,
                s.question_count,
                s.created_at,
                uc.id AS posted_by_id,
                up.username AS posted_by_username,
                uc.email AS posted_by_email,
                up.created_at AS posted_by_created_at
            FROM statistic s
            JOIN user_profile up ON s.posted_by_user_id = up.user_id
            JOIN user_credential uc ON up.user_id = uc.id
            WHERE s.id IN (SELECT statistic_id FROM statistic_tag WHERE tag = ?)
            AND (? = '' OR s.title LIKE ? OR s.description LIKE ?)
            ORDER BY s.created_at DESC
            LIMIT ? OFFSET ?",
        )
        .bind(params.tag)
        .bind(params.search)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(params.limit)
        .bind(params.offset)
        .fetch_all(pool)
        .await?;
        return Ok(results);
    }

    let search_pattern = format!("%{}%", params.search);
    let results = sqlx::query_as(
        "SELECT
            s.id,
            s.title,
            s.description,
            s.upvotes,
            s.downvotes,
            s.question_count,
            s.created_at,
            uc.id AS posted_by_id,
            up.username AS posted_by_username,
            uc.email AS posted_by_email,
            up.created_at AS posted_by_created_at
        FROM statistic s
        JOIN user_profile up ON s.posted_by_user_id = up.user_id
        JOIN user_credential uc ON up.user_id = uc.id
        WHERE (? = '' OR s.title LIKE ? OR s.description LIKE ?)
        ORDER BY s.created_at DESC
        LIMIT ? OFFSET ?",
    )
    .bind(params.search)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .bind(params.limit)
    .bind(params.offset)
    .fetch_all(pool)
    .await?;
    Ok(results)
}

pub async fn count_query(
    db_manager: &DatabaseManager,
    search: &str,
    tag: &str,
) -> DatabaseResult<u64> {
    let pool = db_manager.get_database_pool();

    if !tag.is_empty() {
        let search_pattern = format!("%{search}%");
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM statistic s
             WHERE s.id IN (SELECT statistic_id FROM statistic_tag WHERE tag = ?)
             AND (? = '' OR s.title LIKE ? OR s.description LIKE ?)",
        )
        .bind(tag)
        .bind(search)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_one(pool)
        .await?;
        return Ok(row.0 as u64);
    }

    let search_pattern = format!("%{search}%");
    let row: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM statistic s
         WHERE (? = '' OR s.title LIKE ? OR s.description LIKE ?)",
    )
    .bind(search)
    .bind(&search_pattern)
    .bind(&search_pattern)
    .fetch_one(pool)
    .await?;
    Ok(row.0 as u64)
}
