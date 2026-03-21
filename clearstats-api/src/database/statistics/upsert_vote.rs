use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: DatabaseInteger,
    user_id: DatabaseInteger,
    vote: i16,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    // Remove existing vote contribution
    sqlx::query(
        "UPDATE statistic
         SET upvotes = upvotes - CASE WHEN (SELECT vote FROM statistic_vote WHERE statistic_id = $1 AND user_id = $2) = 1 THEN 1 ELSE 0 END,
             downvotes = downvotes - CASE WHEN (SELECT vote FROM statistic_vote WHERE statistic_id = $3 AND user_id = $4) = -1 THEN 1 ELSE 0 END
         WHERE id = $5",
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
         VALUES ($1, $2, $3)
         ON CONFLICT (statistic_id, user_id) DO UPDATE SET vote = EXCLUDED.vote",
    )
    .bind(statistic_id)
    .bind(user_id)
    .bind(vote)
    .execute(pool)
    .await?;

    // Apply new vote
    if vote == 1 {
        sqlx::query("UPDATE statistic SET upvotes = upvotes + 1 WHERE id = $1")
            .bind(statistic_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE statistic SET downvotes = downvotes + 1 WHERE id = $1")
            .bind(statistic_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}
