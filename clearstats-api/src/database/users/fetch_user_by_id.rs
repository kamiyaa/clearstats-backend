use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct SqlData {
    pub id: DatabaseInteger,
    pub username: String,
    pub email: String,
    pub created_at: DatabaseInteger,
}

pub async fn run_query(
    db_manager: &DatabaseManager,
    user_id: DatabaseInteger,
) -> DatabaseResult<Option<SqlData>> {
    let pool = db_manager.get_database_pool();
    let sql_res = sqlx::query_as(
        "SELECT
            uc.id,
            up.username,
            uc.email,
            up.created_at
        FROM user_profile up
        INNER JOIN user_credential uc ON up.user_id = uc.id
        WHERE up.user_id = ?",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;
    Ok(sql_res)
}
