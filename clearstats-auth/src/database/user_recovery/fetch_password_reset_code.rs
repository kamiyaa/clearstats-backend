use sqlx::FromRow;

use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::{TABLE_USER_PASSWORD_RESET, TABLE_USER_PROFILE};
use shared_lib::database::{DatabaseInteger, DatabaseResult};

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub user_id: DatabaseInteger,
    pub expires_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    username: &str,
    code: &str,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        SELECT
            user_profile.user_id,
            user_profile.username,
            p_reset.expires_at
        FROM
            {TABLE_USER_PASSWORD_RESET} p_reset
        INNER JOIN
            {TABLE_USER_PROFILE} user_profile
        ON
            p_reset.user_id = user_profile.user_id
        WHERE
            p_reset.code = ?
        AND
            user_profile.username = ?
        ;"
    );
    let sql_res = sqlx::query_as(&sql_query)
        .bind(code)
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(sql_res)
}
