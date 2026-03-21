use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub title: String,
    pub description: String,
    pub upvotes: DatabaseInteger,
    pub downvotes: DatabaseInteger,
    pub user_vote: Option<i8>,
    pub question_count: DatabaseInteger,
    pub created_at: DatabaseInteger,
    pub updated_at: DatabaseInteger,
    pub posted_by_id: DatabaseInteger,
    pub posted_by_username: String,
    pub posted_by_created_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: DatabaseInteger,
    user_id: Option<DatabaseInteger>,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();

    let sql_query = "SELECT
        s.id,
        s.title,
        s.description,
        s.upvotes,
        s.downvotes,
        s_vote.vote AS user_vote,
        s.question_count,
        s.created_at,
        s.updated_at,
        up.user_id AS posted_by_id,
        up.username AS posted_by_username,
        up.created_at AS posted_by_created_at
    FROM
        statistic s
    INNER JOIN
        user_profile up
    ON
        s.posted_by_user_id = up.user_id
    LEFT JOIN
        statistic_vote s_vote
    ON
        s_vote.statistic_id = s.id
    AND
        s_vote.user_id = ?
    WHERE
        s.id = ?";

    let sql_res = sqlx::query_as(sql_query)
        .bind(user_id)
        .bind(statistic_id)
        .fetch_optional(pool)
        .await?;
    Ok(sql_res)
}
