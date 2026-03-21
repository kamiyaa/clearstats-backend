use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub statistic_id: DatabaseInteger,
    pub vote: i8,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_ids: &[DatabaseInteger],
    user_id: DatabaseInteger,
) -> DatabaseResult<Vec<SqlData>> {
    if statistic_ids.is_empty() {
        return Ok(vec![]);
    }
    let pool = db_manager.get_database_pool();
    let placeholders = statistic_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT statistic_id, vote FROM statistic_vote WHERE user_id = ? AND statistic_id IN ({placeholders})"
    );
    let mut query = sqlx::query_as(&sql).bind(user_id);
    for id in statistic_ids {
        query = query.bind(id);
    }
    let results = query.fetch_all(pool).await?;
    Ok(results)
}
