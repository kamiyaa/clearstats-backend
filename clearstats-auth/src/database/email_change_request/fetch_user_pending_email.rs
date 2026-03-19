use shared_lib::database::tables::user::TABLE_EMAIL_CHANGE_REQUEST;
use sqlx::FromRow;

use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};

pub async fn run_query(
    db_manager: &DatabaseManager,
    user_id: u64,
) -> DatabaseResult<Option<String>> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "SELECT
            pending_email
        FROM
            {TABLE_EMAIL_CHANGE_REQUEST} 
        WHERE
            user_id = ?
        ;"
    );

    #[derive(Clone, Debug, FromRow)]
    pub struct SqlData {
        pub pending_email: String,
    }
    let pending_email: Option<SqlData> = sqlx::query_as(&sql_query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    Ok(pending_email.map(|s| s.pending_email))
}
