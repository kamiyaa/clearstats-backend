use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: u64,
    pub upvotes: u64,
    pub downvotes: u64,
    pub question_count: u64,
    pub created_at: u64,
    pub updated_at: u64,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: u64,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();
    let sql_res = sqlx::query_as(
        "SELECT
            s.id,
            s.upvotes,
            s.downvotes,
            s.question_count,
            s.created_at,
            s.updated_at
        FROM
            statistic s
        WHERE
            s.id = ?",
    )
    .bind(statistic_id)
    .fetch_optional(pool)
    .await?;
    Ok(sql_res)
}
