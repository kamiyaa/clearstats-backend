use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::{DatabaseInteger, DatabaseResult};

pub async fn run_query(
    db_manager: &DatabaseManager,
    statistic_id: DatabaseInteger,
    user_id: DatabaseInteger,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    // Remove vote contribution from counts
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

    sqlx::query("DELETE FROM statistic_vote WHERE statistic_id = $1 AND user_id = $2")
        .bind(statistic_id)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
