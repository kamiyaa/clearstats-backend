use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub affiliation: Option<String>,
}

pub async fn run_query(db_manager: &DatabaseManager, q: &str) -> DatabaseResult<Vec<SqlData>> {
    let pool = db_manager.get_database_pool();
    let pattern = format!("%{q}%");
    let results = sqlx::query_as(
        "SELECT id, name, bio, avatar_url, affiliation
        FROM author
        WHERE
            name LIKE $1 OR affiliation LIKE $2
        LIMIT 20",
    )
    .bind(&pattern)
    .bind(&pattern)
    .fetch_all(pool)
    .await?;
    Ok(results)
}
