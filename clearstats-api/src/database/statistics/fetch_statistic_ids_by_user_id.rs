use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    user_id: DatabaseInteger,
) -> DatabaseResult<Vec<SqlData>> {
    let pool = db_manager.get_database_pool();
    let results = sqlx::query_as(
        "SELECT id FROM statistic WHERE posted_by_user_id = ? ORDER BY created_at DESC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(results)
}
