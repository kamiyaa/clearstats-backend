pub mod manager;
pub mod tables;

use axum::http::StatusCode;
use sqlx::migrate::Migrator;
use sqlx::{MySql, pool::PoolOptions};

use crate::error::{AppServerResult, ServerErrorResponse};

pub static DEFAULT_MIGRATOR: Migrator = sqlx::migrate!("../migrations");

pub type DatabasePool = sqlx::MySqlPool;
pub type DatabaseTransaction = sqlx::Transaction<'static, MySql>;
pub type DatabaseResult<T = ()> = Result<T, sqlx::Error>;

pub async fn connect_to_database(database_url: &str) -> AppServerResult<DatabasePool> {
    println!("Connecting to database...");
    let pool = PoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|err| {
            let error_msg = "Failed to connect to database!";
            tracing::error!(?err, "{error_msg}");
            ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1234,
                error_msg.to_string(),
            )
        })?;
    println!("Connected!");
    Ok(pool)
}
