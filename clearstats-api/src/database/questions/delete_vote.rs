use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(
    db_manager: &DatabaseManager,
    question_id: DatabaseInteger,
    user_id: DatabaseInteger,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    // Remove vote contribution from counts
    sqlx::query(
        "UPDATE question
         SET upvotes = upvotes - IF((SELECT vote FROM question_vote WHERE question_id = ? AND user_id = ?) = 1, 1, 0),
             downvotes = downvotes - IF((SELECT vote FROM question_vote WHERE question_id = ? AND user_id = ?) = -1, 1, 0)
         WHERE id = ?",
    )
    .bind(question_id)
    .bind(user_id)
    .bind(question_id)
    .bind(user_id)
    .bind(question_id)
    .execute(pool)
    .await?;

    sqlx::query("DELETE FROM question_vote WHERE question_id = ? AND user_id = ?")
        .bind(question_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
