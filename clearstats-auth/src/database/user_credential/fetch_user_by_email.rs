use shared_lib::database::tables::user::{TABLE_USER_CREDENTIAL, TABLE_USER_PROFILE};
use shared_lib::database::{DatabaseBoolean, DatabaseInteger, DatabaseResult};
use sqlx::FromRow;

use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub email_verified: DatabaseBoolean,
    pub first_name: String,
    pub last_name: String,
    pub icon_hash: Option<String>,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    email: &str,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        SELECT
            user_cred.id,
            user_cred.password_hash,
            user_cred.salt,
            user_cred.email_verified,
            user_profile.username,
            user_profile.first_name,
            user_profile.last_name,
            user_profile.icon_hash
        FROM
            {TABLE_USER_CREDENTIAL} user_cred
        INNER JOIN
            {TABLE_USER_PROFILE} user_profile
        ON
            user_cred.id = user_profile.user_id
        AND
            user_cred.email = ?
    ;"
    );
    let sql_res = sqlx::query_as(&sql_query)
        .bind(email)
        .fetch_optional(pool)
        .await?;
    Ok(sql_res)
}
