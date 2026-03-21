use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};

pub async fn run_query(
    db_manager: &DatabaseManager,
    question_id: DatabaseInteger,
    user_id: DatabaseInteger,
    vote: i16,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    // Remove existing vote contribution
    sqlx::query(
        "UPDATE question
         SET upvotes = upvotes - CASE WHEN (SELECT vote FROM question_vote WHERE question_id = $1 AND user_id = $2) = 1 THEN 1 ELSE 0 END,
             downvotes = downvotes - CASE WHEN (SELECT vote FROM question_vote WHERE question_id = $3 AND user_id = $4) = -1 THEN 1 ELSE 0 END
         WHERE id = $5",
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
         VALUES ($1, $2, $3)
         ON CONFLICT (question_id, user_id) DO UPDATE SET vote = EXCLUDED.vote",
    )
    .bind(question_id)
    .bind(user_id)
    .bind(vote)
    .execute(pool)
    .await?;

    if vote == 1 {
        sqlx::query("UPDATE question SET upvotes = upvotes + 1 WHERE id = $1")
            .bind(question_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE question SET downvotes = downvotes + 1 WHERE id = $1")
            .bind(question_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}
