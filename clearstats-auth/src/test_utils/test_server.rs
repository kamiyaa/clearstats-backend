use axum::Router;
use shared_lib::{
    database::{DatabasePool, manager::DatabaseManager},
    server::cors::all_origin_cors,
    test_utils::{gcp::generate_test_gcp_access_token_arc, test_channel::create_test_channel},
};

use crate::{
    ServerRouter, config::AppConfig, queue::types::ServerMessageReceiver, state::AppState,
};

#[allow(dead_code)]
pub struct TestServer {
    pub app: Router,
    pub rx: ServerMessageReceiver,
    pub db_manager: DatabaseManager,
}

pub fn setup_test_server(
    config: &AppConfig,
    pool: DatabasePool,
    router: ServerRouter,
) -> TestServer {
    let gcp_service_account_token = generate_test_gcp_access_token_arc();

    let db_manager = DatabaseManager::new(pool);

    let (tx, rx) = create_test_channel();
    let app_state = AppState {
        config: config.clone(),
        db_manager: db_manager.clone(),
        gcp_service_account_token,
        message_tx: tx,
    };

    let app = ServerRouter::new()
        .merge(router)
        .layer(all_origin_cors())
        .with_state(app_state);

    TestServer {
        app,
        rx,
        db_manager,
    }
}

#[allow(dead_code)]
pub struct TestAppState {
    pub app_state: AppState,
    pub rx: ServerMessageReceiver,
    pub db_manager: DatabaseManager,
}

pub fn setup_app_state(config: &AppConfig, pool: DatabasePool) -> TestAppState {
    let gcp_service_account_token = generate_test_gcp_access_token_arc();

    let db_manager = DatabaseManager::new(pool);

    let (tx, rx) = create_test_channel();
    let app_state = AppState {
        config: config.clone(),
        db_manager: db_manager.clone(),
        gcp_service_account_token,
        message_tx: tx,
    };
    TestAppState {
        app_state,
        rx,
        db_manager,
    }
}
