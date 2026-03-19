use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: u64,
    pub username: String,
    pub password_hash: String,
    pub salt: String,
    pub email_verified: u8,
    pub first_name: String,
    pub last_name: String,
    pub icon_hash: Option<String>,
    pub created_at: u64,
    pub email: String,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    email: &str,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();
    let sql_res = sqlx::query_as(
        "SELECT
            uc.id,
            uc.email,
            uc.password_hash,
            uc.salt,
            uc.email_verified,
            up.username,
            up.first_name,
            up.last_name,
            up.icon_hash,
            up.created_at
        FROM user_credential uc
        INNER JOIN user_profile up ON uc.id = up.user_id
        WHERE uc.email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(sql_res)
}
