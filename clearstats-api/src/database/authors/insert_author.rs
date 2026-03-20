use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

pub struct SqlData<'a> {
    pub name: &'a str,
    pub bio: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
    pub affiliation: Option<&'a str>,
}

#[derive(Clone, Debug, FromRow)]
pub struct AuthorRow {
    pub id: u64,
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub affiliation: Option<String>,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    data: &SqlData<'_>,
) -> DatabaseResult<AuthorRow> {
    let pool = db_manager.get_database_pool();

    let res =
        sqlx::query("INSERT INTO author (name, bio, avatar_url, affiliation) VALUES (?, ?, ?, ?)")
            .bind(data.name)
            .bind(data.bio)
            .bind(data.avatar_url)
            .bind(data.affiliation)
            .execute(pool)
            .await?;

    let author_id = res.last_insert_id();

    let row: AuthorRow =
        sqlx::query_as("SELECT id, name, bio, avatar_url, affiliation FROM author WHERE id = ?")
            .bind(author_id)
            .fetch_one(pool)
            .await?;

    Ok(row)
}
