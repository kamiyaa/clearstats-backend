use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_USER_PASSWORD_RESET;
use shared_lib::database::{DatabaseInteger, DatabaseResult};

pub async fn run_query(
    db_manager: &DatabaseManager,
    user_id: DatabaseInteger,
    password_reset_code: &str,
    expires_at: DatabaseInteger,
) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        INSERT INTO {TABLE_USER_PASSWORD_RESET}
            (user_id, code, expires_at)
        VALUES
            ($1, $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET
            code = EXCLUDED.code,
            expires_at = EXCLUDED.expires_at
        ;"
    );

    let _res = sqlx::query(&sql_query)
        .bind(user_id)
        .bind(password_reset_code)
        .bind(expires_at)
        .execute(pool)
        .await?;
    Ok(())
}
