use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::TABLE_USER_CREDENTIAL;
use shared_lib::database::{DatabaseInteger, DatabaseResult};

#[derive(Clone, Debug)]
pub struct SqlQuery {
    pub user_id: DatabaseInteger,
    pub new_password_hash: String,
    pub new_salt: String,
}

pub async fn run_query(db_manager: &DatabaseManager, data: &SqlQuery) -> DatabaseResult<u64> {
    let pool = db_manager.get_database_pool();

    let sql_query = format!(
        "
        UPDATE
            {TABLE_USER_CREDENTIAL}
        SET
            password_hash = ?,
            salt = ?
        WHERE
            id = ?
        ;
        "
    );
    let sql_res = sqlx::query(&sql_query)
        .bind(&data.new_password_hash)
        .bind(&data.new_salt)
        .bind(data.user_id)
        .execute(pool)
        .await?;
    Ok(sql_res.rows_affected())
}
