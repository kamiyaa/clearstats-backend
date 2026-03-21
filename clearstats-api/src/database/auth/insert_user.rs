use shared_lib::database::{DatabaseInteger, DatabaseResult};
use shared_lib::database::manager::{DatabaseManager, DatabaseManagerTrait};
use shared_lib::types::database::SqlId;

pub struct SqlData {
    pub email: String,
    pub password_hash: String,
    pub salt: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DatabaseInteger,
}

pub async fn run_query(db_manager: &DatabaseManager, data: &SqlData) -> DatabaseResult<DatabaseInteger> {
    let pool = db_manager.get_database_pool();

    let res: SqlId =
        sqlx::query_as("INSERT INTO user_credential
            (email, password_hash, salt)
        VALUES (?, ?, ?)
        RETURNING id;")
            .bind(&data.email)
            .bind(&data.password_hash)
            .bind(&data.salt)
            .fetch_one(pool)
            .await?;

    let user_id = res.id;

    sqlx::query(
        "INSERT INTO user_profile (user_id, username, first_name, last_name, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(&data.username)
    .bind(&data.first_name)
    .bind(&data.last_name)
    .bind(data.created_at)
    .bind(data.created_at)
    .execute(pool)
    .await?;

    Ok(user_id)
}
