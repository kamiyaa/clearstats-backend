use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub statistic_id: DatabaseInteger,
    pub url: String,
    pub filename: String,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_ids: &[DatabaseInteger],
) -> DatabaseResult<Vec<SqlData>> {
    if statistic_ids.is_empty() {
        return Ok(vec![]);
    }
    let pool = db_manager.get_database_pool();
    let placeholders = (1..=statistic_ids.len())
        .map(|i| format!("${}", i))
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
