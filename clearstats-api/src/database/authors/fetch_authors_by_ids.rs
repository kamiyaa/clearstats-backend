use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: u64,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub affiliation: Option<String>,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    author_ids: &[u64],
) -> DatabaseResult<Vec<SqlData>> {
    if author_ids.is_empty() {
        return Ok(vec![]);
    }
    let pool = db_manager.get_database_pool();
    let placeholders = author_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT id, name, bio, avatar_url, affiliation FROM author WHERE id IN ({placeholders})"
    );
    let mut query = sqlx::query_as(&sql);
    for id in author_ids {
        query = query.bind(id);
    }
    let results = query.fetch_all(pool).await?;
    Ok(results)
}
