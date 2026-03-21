use shared_lib::database::DatabaseResult;
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_EMAIL_CHANGE_REQUEST;

pub async fn run_query(db_manager: &DatabaseManager, email: &str) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        DELETE FROM {TABLE_EMAIL_CHANGE_REQUEST}
        WHERE
            pending_email = $1
        ;"
    );

    let _res = sqlx::query(&sql_query).bind(email).execute(pool).await?;
    Ok(())
}
