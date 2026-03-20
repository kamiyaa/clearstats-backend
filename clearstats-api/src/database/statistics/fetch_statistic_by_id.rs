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
    pub updated_at: u64,
    pub posted_by_id: u64,
    pub posted_by_username: String,
    pub posted_by_created_at: u64,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: u64,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();
    let sql_res = sqlx::query_as(
        "SELECT
            s.id,
            s.title,
            s.description,
            s.upvotes,
            s.downvotes,
            s.question_count,
            s.created_at,
            s.updated_at,
            up.user_id AS posted_by_id,
            up.username AS posted_by_username,
            up.created_at AS posted_by_created_at
        FROM
            statistic s
        INNER JOIN
            user_profile up
        ON
            s.posted_by_user_id = up.user_id
        WHERE
            s.id = ?",
    )
    .bind(statistic_id)
    .fetch_optional(pool)
    .await?;
    Ok(sql_res)
}
