use shared_lib::database::tables::user::TABLE_USER_CREDENTIAL;
use sqlx::FromRow;

use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(
    db_manager: &DatabaseManager,
    user_id: DatabaseInteger,
) -> DatabaseResult<Option<String>> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "SELECT
            user_cred.email
        FROM
            {TABLE_USER_CREDENTIAL} user_cred
        WHERE
            user_cred.id = ?
        ;"
    );

    #[derive(Clone, Debug, FromRow)]
    pub struct SqlData {
        pub email: String,
    }
    let user_email: Option<SqlData> = sqlx::query_as(&sql_query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(user_email.map(|s| s.email))
}
