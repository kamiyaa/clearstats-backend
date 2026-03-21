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

pub async fn run_query(
    db_manager: &DatabaseManager,
    author_id: DatabaseInteger,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();
    let sql_res =
        sqlx::query_as("SELECT id, name, bio, avatar_url, affiliation FROM author WHERE id = $1")
            .bind(author_id)
            .fetch_optional(pool)
            .await?;
    Ok(sql_res)
}
