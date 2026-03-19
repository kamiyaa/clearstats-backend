use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: u64,
    pub statistic_id: u64,
    pub body: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub created_at: u64,
    pub posted_by_id: u64,
    pub posted_by_username: String,
    pub posted_by_email: String,
    pub posted_by_created_at: u64,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: u64,
) -> DatabaseResult<Vec<SqlData>> {
    let pool = db_manager.get_database_pool();
    let results = sqlx::query_as(
        "SELECT
            q.id,
            q.statistic_id,
            q.body,
            q.upvotes,
            q.downvotes,
            q.created_at,
            uc.id AS posted_by_id,
            up.username AS posted_by_username,
            uc.email AS posted_by_email,
            up.created_at AS posted_by_created_at
        FROM question q
        JOIN user_profile up ON q.posted_by_user_id = up.user_id
        JOIN user_credential uc ON up.user_id = uc.id
        WHERE q.statistic_id = ?
        ORDER BY q.created_at ASC",
    )
    .bind(statistic_id)
    .fetch_all(pool)
    .await?;
    Ok(results)
}
