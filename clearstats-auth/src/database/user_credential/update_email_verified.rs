use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_USER_CREDENTIAL;

pub async fn run_query(
    db_manager: &DatabaseManager,
    email: &str,
    verified: bool,
) -> DatabaseResult<u64> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
UPDATE
    {TABLE_USER_CREDENTIAL}
SET
    email_verified = $1
WHERE
    email = $2
;"
    );

    let sql_res = sqlx::query(&sql_query)
        .bind(verified)
        .bind(email)
        .execute(pool)
        .await?;
    Ok(sql_res.rows_affected())
}
