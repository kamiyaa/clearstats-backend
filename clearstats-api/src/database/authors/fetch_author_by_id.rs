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
    author_id: u64,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();
    let sql_res =
        sqlx::query_as("SELECT id, name, bio, avatar_url, affiliation FROM author WHERE id = ?")
            .bind(author_id)
            .fetch_optional(pool)
            .await?;
    Ok(sql_res)
}
