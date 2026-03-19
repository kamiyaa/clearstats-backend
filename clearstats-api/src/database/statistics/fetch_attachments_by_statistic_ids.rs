use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: u64,
    pub statistic_id: u64,
    pub url: String,
    pub filename: String,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_ids: &[u64],
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
        "SELECT id, statistic_id, url, filename FROM statistic_attachment WHERE statistic_id IN ({placeholders})"
    );
    let mut query = sqlx::query_as(&sql);
    for id in statistic_ids {
        query = query.bind(id);
    }
    let results = query.fetch_all(pool).await?;
    Ok(results)
}
