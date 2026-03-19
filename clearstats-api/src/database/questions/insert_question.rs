use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

pub struct SqlData<'a> {
    pub statistic_id: u64,
    pub body: &'a str,
    pub posted_by_user_id: u64,
    pub created_at: u64,
}

#[derive(Clone, Debug, FromRow)]
pub struct QuestionRow {
    pub id: u64,
    pub statistic_id: u64,
    pub body: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub created_at: u64,
    pub posted_by_id: u64,
    pub posted_by_username: String,
    pub posted_by_email: String,
    pub posted_by_created_at: u64,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    data: &SqlData<'_>,
) -> DatabaseResult<QuestionRow> {
    let pool = db_manager.get_database_pool();

    let res = sqlx::query(
        "INSERT INTO question (statistic_id, body, posted_by_user_id, created_at)
         VALUES (?, ?, ?, ?)",
    )
    .bind(data.statistic_id)
    .bind(data.body)
    .bind(data.posted_by_user_id)
    .bind(data.created_at)
    .execute(pool)
    .await?;

    let question_id = res.last_insert_id();

    sqlx::query("UPDATE statistic SET question_count = question_count + 1 WHERE id = ?")
        .bind(data.statistic_id)
        .execute(pool)
        .await?;

    let row: QuestionRow = sqlx::query_as(
        "SELECT
            q.id,
            q.statistic_id,
            q.body,
            q.upvotes,
            q.downvotes,
            q.created_at,
            uc.id AS posted_by_id,
            up.username AS posted_by_username,
            uc.email AS posted_by_email,
            up.created_at AS posted_by_created_at
        FROM question q
        JOIN user_profile up ON q.posted_by_user_id = up.user_id
        JOIN user_credential uc ON up.user_id = uc.id
        WHERE q.id = ?",
    )
    .bind(question_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
