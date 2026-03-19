use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_EMAIL_CHANGE_REQUEST;

pub async fn run_query(
    db_manager: &DatabaseManager,
    email: &str,
    user_id: u64,
    verification_code: &str,
    created_at: u64,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        INSERT INTO {TABLE_EMAIL_CHANGE_REQUEST}
            (user_id, pending_email, verification_code, created_at)
        VALUES
            (?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
            pending_email = ?,
            verification_code = ?,
            created_at = ?
        ;"
    );

    let _res = sqlx::query(&sql_query)
        .bind(user_id)
        .bind(email)
        .bind(verification_code)
        .bind(created_at)
        .bind(email)
        .bind(verification_code)
        .bind(created_at)
        .execute(pool)
        .await?;
    Ok(())
}
