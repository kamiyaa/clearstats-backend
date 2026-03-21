use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub question_id: DatabaseInteger,
    pub vote: i8,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    question_ids: &[DatabaseInteger],
    user_id: DatabaseInteger,
) -> DatabaseResult<Vec<SqlData>> {
    if question_ids.is_empty() {
        return Ok(vec![]);
    }
    let pool = db_manager.get_database_pool();
    let placeholders = question_ids
        .iter()
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT question_id, vote FROM question_vote WHERE user_id = ? AND question_id IN ({placeholders})"
    );
    let mut query = sqlx::query_as(&sql).bind(user_id);
    for id in question_ids {
        query = query.bind(id);
    }
    let results = query.fetch_all(pool).await?;
    Ok(results)
}
