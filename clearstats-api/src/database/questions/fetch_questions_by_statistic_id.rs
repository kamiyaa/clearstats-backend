use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub statistic_id: DatabaseInteger,
    pub body: String,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub created_at: DatabaseInteger,
    pub posted_by_id: DatabaseInteger,
    pub posted_by_username: String,
    pub posted_by_email: String,
    pub posted_by_created_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: DatabaseInteger,
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
            up.user_id AS posted_by_id,
            up.username AS posted_by_username,
            up.created_at AS posted_by_created_at
        FROM
            question q
        INNER JOIN
            user_profile up
        ON
            q.posted_by_user_id = up.user_id
        WHERE
            q.statistic_id = ?
        ORDER BY
            q.created_at ASC;",
    )
    .bind(statistic_id)
    .fetch_all(pool)
    .await?;
    Ok(results)
}
