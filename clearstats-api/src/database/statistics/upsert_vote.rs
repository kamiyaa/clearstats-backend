use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: u64,
    user_id: u64,
    vote: i8,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    // Remove existing vote contribution
    sqlx::query(
        "UPDATE statistic
         SET upvotes = upvotes - IF((SELECT vote FROM statistic_vote WHERE statistic_id = ? AND user_id = ?) = 1, 1, 0),
             downvotes = downvotes - IF((SELECT vote FROM statistic_vote WHERE statistic_id = ? AND user_id = ?) = -1, 1, 0)
         WHERE id = ?",
    )
    .bind(statistic_id)
    .bind(user_id)
    .bind(statistic_id)
    .bind(user_id)
    .bind(statistic_id)
    .execute(pool)
    .await?;

    sqlx::query(
        "INSERT INTO statistic_vote (statistic_id, user_id, vote)
         VALUES (?, ?, ?)
         ON DUPLICATE KEY UPDATE vote = VALUES(vote)",
    )
    .bind(statistic_id)
    .bind(user_id)
    .bind(vote)
    .execute(pool)
    .await?;

    // Apply new vote
    if vote == 1 {
        sqlx::query("UPDATE statistic SET upvotes = upvotes + 1 WHERE id = ?")
            .bind(statistic_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE statistic SET downvotes = downvotes + 1 WHERE id = ?")
            .bind(statistic_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}
