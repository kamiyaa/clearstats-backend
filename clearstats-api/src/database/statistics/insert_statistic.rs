use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::types::database::SqlId;

pub struct SqlData<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub posted_by_user_id: DatabaseInteger,
    pub tags: &'a [String],
    pub sources: &'a [(String, Option<String>)],
    pub attachments: &'a [(String, String)],
    pub author_ids: &'a [DatabaseInteger],
    pub created_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    data: &SqlData<'_>,
) -> DatabaseResult<DatabaseInteger> {
    let pool = db_manager.get_database_pool();

    let mut tx = pool.begin().await?;

    let res: SqlId = sqlx::query_as(
        "INSERT INTO statistic (title, description, posted_by_user_id, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5)
         RETURNING id;",
    )
    .bind(data.title)
    .bind(data.description)
    .bind(data.posted_by_user_id)
    .bind(data.created_at)
    .bind(data.created_at)
    .fetch_one(&mut *tx)
    .await?;

    let statistic_id = res.id;

    for tag in data.tags {
        sqlx::query("INSERT INTO statistic_tag (statistic_id, tag) VALUES ($1, $2)")
            .bind(statistic_id)
            .bind(tag)
            .execute(&mut *tx)
            .await?;
    }

    for (url, title) in data.sources {
        sqlx::query("INSERT INTO statistic_source (statistic_id, url, title) VALUES ($1, $2, $3)")
            .bind(statistic_id)
            .bind(url)
            .bind(title.as_deref())
            .execute(&mut *tx)
            .await?;
    }

    for (url, filename) in data.attachments {
        sqlx::query(
            "INSERT INTO statistic_attachment (statistic_id, url, filename) VALUES ($1, $2, $3)",
        )
        .bind(statistic_id)
        .bind(url)
        .bind(filename)
        .execute(&mut *tx)
        .await?;
    }

    for author_id in data.author_ids {
        sqlx::query("INSERT INTO statistic_author (statistic_id, author_id) VALUES ($1, $2)")
            .bind(statistic_id)
            .bind(author_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    Ok(statistic_id)
}
