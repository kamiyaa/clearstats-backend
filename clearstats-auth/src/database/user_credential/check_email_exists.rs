use shared_lib::database::DatabaseResult;
use shared_lib::database::tables::user::{TABLE_EMAIL_CHANGE_REQUEST, TABLE_USER_CREDENTIAL};
use shared_lib::types::database::SqlCount;

use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(db_manager: &DatabaseManager, email: &str) -> DatabaseResult<i64> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        SELECT
            COUNT(email) AS count
        FROM
            {TABLE_USER_CREDENTIAL}
        WHERE
            email = ?
        UNION
        SELECT
            COUNT(pending_email) AS pending_email_count
        FROM
            {TABLE_EMAIL_CHANGE_REQUEST}
        WHERE pending_email = ?
    ;"
    );
    let sql_res: SqlCount = sqlx::query_as(&sql_query)
        .bind(email)
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(sql_res.count)
}
