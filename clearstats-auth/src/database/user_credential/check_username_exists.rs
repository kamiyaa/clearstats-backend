use shared_lib::database::DatabaseResult;
use shared_lib::database::tables::user::TABLE_USER_PROFILE;
use shared_lib::types::database::SqlCount;

use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(db_manager: &DatabaseManager, username: &str) -> DatabaseResult<i64> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        SELECT
            COUNT(*) AS count
        FROM
            {TABLE_USER_PROFILE}
        WHERE
            username = ?
    ;"
    );
    let sql_res: SqlCount = sqlx::query_as(&sql_query)
        .bind(username)
        .fetch_one(pool)
        .await?;
    Ok(sql_res.count)
}
