use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::types::database::SqlId;
use sqlx::FromRow;

pub struct SqlData<'a> {
    pub statistic_id: DatabaseInteger,
    pub body: &'a str,
    pub posted_by_user_id: DatabaseInteger,
    pub created_at: DatabaseInteger,
}

#[derive(Clone, Debug, FromRow)]
pub struct QuestionRow {
    pub id: DatabaseInteger,
    pub statistic_id: DatabaseInteger,
    pub body: String,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub created_at: DatabaseInteger,
    pub posted_by_id: DatabaseInteger,
    pub posted_by_username: String,
    pub posted_by_created_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    data: &SqlData<'_>,
) -> DatabaseResult<QuestionRow> {
    let pool = db_manager.get_database_pool();

    let res: SqlId = sqlx::query_as(
        "INSERT INTO question (statistic_id, body, posted_by_user_id, created_at)
         VALUES (?, ?, ?, ?)
         RETURNING id;",
    )
    .bind(data.statistic_id)
    .bind(data.body)
    .bind(data.posted_by_user_id)
    .bind(data.created_at)
    .fetch_one(pool)
    .await?;

    let question_id = res.id;

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
            up.user_id AS posted_by_id,
            up.username AS posted_by_username,
            up.created_at AS posted_by_created_at
        FROM
            question q
        INNER JOIN
            user_profile up
        ON
            q.posted_by_user_id = up.user_id
        WHERE
            q.id = ?",
    )
    .bind(question_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
