use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::{TABLE_EMAIL_CHANGE_REQUEST, TABLE_USER_CREDENTIAL};
use shared_lib::database::{DatabaseInteger, DatabaseResult};

pub async fn run_query(
    db_manager: &DatabaseManager,
    user_id: DatabaseInteger,
    new_email: &str,
) -> DatabaseResult<u64> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "UPDATE
            {TABLE_USER_CREDENTIAL}
        INNER JOIN
            {TABLE_EMAIL_CHANGE_REQUEST}
        ON
            {TABLE_USER_CREDENTIAL}.id = {TABLE_EMAIL_CHANGE_REQUEST}.user_id
        SET
            {TABLE_USER_CREDENTIAL}.email = {TABLE_EMAIL_CHANGE_REQUEST}.pending_email
        WHERE
            {TABLE_USER_CREDENTIAL}.id = ?
        AND
            {TABLE_EMAIL_CHANGE_REQUEST}.pending_email = ?
        ;"
    );

    let sql_res = sqlx::query(&sql_query)
        .bind(user_id as i64)
        .bind(new_email)
        .execute(pool)
        .await?;
    Ok(sql_res.rows_affected())
}
