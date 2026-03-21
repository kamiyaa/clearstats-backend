use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_USER_PASSWORD_RESET;
use shared_lib::database::{DatabaseInteger, DatabaseResult};

pub async fn run_query(db_manager: &DatabaseManager, user_id: DatabaseInteger) -> DatabaseResult {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        DELETE FROM
            {TABLE_USER_PASSWORD_RESET}
        WHERE
            user_id = $1
        ;"
    );

    let _res = sqlx::query(&sql_query).bind(user_id).execute(pool).await?;
    Ok(())
}
