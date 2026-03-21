use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::database::tables::user::{TABLE_USER_CREDENTIAL, TABLE_USER_PROFILE};
use shared_lib::types::database::SqlId;

pub struct SqlData {
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub email_verified: bool,

    pub username: String,
    pub first_name: String,
    pub last_name: String,

    pub created_at: DatabaseInteger,
    pub updated_at: DatabaseInteger,
}

pub async fn run_query(db_manager: &DatabaseManager, data: &SqlData) -> DatabaseResult<DatabaseInteger> {
    let pool = db_manager.get_database_pool();

    let mut tx = pool.begin().await?;

    let sql_query = format!(
        "
        INSERT INTO {TABLE_USER_CREDENTIAL}
            (email, password_hash, salt, email_verified)
        VALUES
            (?, ?, ?, ?)
        RETURNING id;
        "
    );
    let sql_res: SqlId = sqlx::query_as(&sql_query)
        .bind(&data.email)
        .bind(&data.password_hash)
        .bind(&data.salt)
        .bind(data.email_verified)
        .fetch_one(&mut *tx)
        .await?;

    let user_id = sql_res.id;
    let sql_query = format!(
        "
        INSERT INTO {TABLE_USER_PROFILE}
            (user_id, username, first_name, last_name,
                created_at, updated_at)
        VALUES
            (?, ?, ?, ?,
                ?, ?)
    ;"
    );
    let sql_res = sqlx::query(&sql_query)
        .bind(user_id)
        .bind(&data.username)
        .bind(&data.first_name)
        .bind(&data.last_name)
        .bind(data.created_at)
        .bind(data.updated_at)
        .execute(&mut *tx)
        .await?;

    if sql_res.rows_affected() == 0 {
        tx.rollback().await?;
        return Err(sqlx::Error::RowNotFound);
    }

    tx.commit().await?;
    Ok(user_id)
}
