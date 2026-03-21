use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub question_count: DatabaseInteger,
    pub created_at: DatabaseInteger,
    pub updated_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: DatabaseInteger,
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
            s.id = $1",
    )
    .bind(statistic_id)
    .fetch_optional(pool)
    .await?;
    Ok(sql_res)
}
