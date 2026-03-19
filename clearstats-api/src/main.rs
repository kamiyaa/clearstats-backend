mod api;
mod config;
mod database;
mod helpers;
mod state;
mod types;
mod utils;

use std::env;

use axum::{Router, http::StatusCode, middleware, routing::get};

use shared_lib::database::manager::DatabaseManager;
use shared_lib::logging::setup_tracing_subscriber_registry;
use shared_lib::{
    error::{AppServerResult, ServerErrorResponse},
    server::{HOST, middlewares},
};

use crate::{config::AppConfig, state::AppState};

const PORT: u16 = 8204;

pub type ServerRouter = Router<AppState>;

async fn run_server() -> AppServerResult {
    let config = AppConfig::from_env()?;
    setup_tracing_subscriber_registry(&config);
    start_server(config).await?;
    Ok(())
}

async fn start_server(config: AppConfig) -> AppServerResult {
    let database_pool = shared_lib::database::connect_to_database(&config.database_url).await?;
    let db_manager = DatabaseManager::new(database_pool);

    let app_state = AppState { config, db_manager };

    let app = ServerRouter::new()
        .route("/health", get(shared_lib::server::health::health_check))
        .merge(api::router())
        .layer(middleware::from_fn(
            middlewares::request_logger::structured_logger,
        ))
        .with_state(app_state);

    let host = env::var("HOST").unwrap_or_else(|_| {
        tracing::info!("No HOST env var found, defaulting to {HOST}");
        HOST.to_string()
    });
    let port = env::var("PORT").unwrap_or_else(|_| {
        tracing::info!("No PORT env var found, defaulting to {PORT}");
        PORT.to_string()
    });

    let url = format!("{host}:{port}");
    tracing::info!(url = url, "Serving to");
    let listener = tokio::net::TcpListener::bind(url)
        .await
        .expect("Failed to bind to TCP port");
    axum::serve(listener, app).await.map_err(|err| {
        let error_msg = "Failed to bind to address";
        tracing::error!(?err, "{error_msg}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1000,
            error_msg.to_string(),
        )
    })?;

    Ok(())
}

#[tokio::main]
async fn main() {
    match run_server().await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Fatal Error: {e:#?}");
        }
    }
}
