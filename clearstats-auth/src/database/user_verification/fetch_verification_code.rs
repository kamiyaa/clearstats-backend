use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_USER_EMAIL_VERIFTCATION;

pub async fn run_query(
    db_manager: &DatabaseManager,
    email: &str,
    verification_code: &str,
) -> DatabaseResult<bool> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        SELECT
            verification_code
        FROM
            {TABLE_USER_EMAIL_VERIFTCATION}
        WHERE
            email = ?
        AND
            verification_code = ?
        ;"
    );
    let sql_res = sqlx::query(&sql_query)
        .bind(email)
        .bind(verification_code)
        .fetch_optional(pool)
        .await?;

    Ok(sql_res.is_some())
}
