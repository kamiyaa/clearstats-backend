use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(
    db_manager: &DatabaseManager,
    question_id: DatabaseInteger,
    user_id: DatabaseInteger,
    vote: i8,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    // Remove existing vote contribution
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

    sqlx::query(
        "INSERT INTO question_vote (question_id, user_id, vote)
         VALUES (?, ?, ?)
         ON DUPLICATE KEY UPDATE vote = VALUES(vote)",
    )
    .bind(question_id)
    .bind(user_id)
    .bind(vote)
    .execute(pool)
    .await?;

    if vote == 1 {
        sqlx::query("UPDATE question SET upvotes = upvotes + 1 WHERE id = ?")
            .bind(question_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE question SET downvotes = downvotes + 1 WHERE id = ?")
            .bind(question_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}
