use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_USER_EMAIL_VERIFTCATION;

pub async fn run_query(
    db_manager: &DatabaseManager,
    email: &str,
    verification_code: &str,
    created_at: u64,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        INSERT INTO {TABLE_USER_EMAIL_VERIFTCATION}
            (email, verification_code, created_at)
        VALUES
            (?, ?, ?)
        ON DUPLICATE KEY UPDATE
            verification_code = ?,
            created_at = ?
        ;"
    );

    let _res = sqlx::query(&sql_query)
        .bind(email)
        .bind(verification_code)
        .bind(created_at)
        .bind(verification_code)
        .bind(created_at)
        .execute(pool)
        .await?;
    Ok(())
}
