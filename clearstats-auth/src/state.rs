use std::{sync::Arc, time::SystemTime};

use tokio::sync::RwLock;

use shared_lib::{
    error::AppServerResult,
    integrations::gcp::{self, ServiceAccountAuthToken},
};

use shared_lib::database::manager::DatabaseManager;

use crate::{config::AppConfig, queue::types::ServerMessage};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_manager: DatabaseManager,
    pub config: AppConfig,
    pub gcp_service_account_token: Arc<RwLock<ServiceAccountAuthToken>>,
    pub message_tx: tokio::sync::mpsc::Sender<ServerMessage>,
}

#[allow(dead_code)]
impl AppState {
    pub async fn get_gcp_service_account_token(&mut self) -> AppServerResult<String> {
        let rw_res = self.gcp_service_account_token.read().await;
        let now = SystemTime::now();
        if now >= rw_res.expires_at {
            let new_account_token = gcp::fetch_service_account_auth_token().await?;
            let mut rw_res = self.gcp_service_account_token.write().await;
            *rw_res = new_account_token.into();
            Ok(rw_res.access_token.to_owned())
        } else {
            Ok(rw_res.access_token.to_owned())
        }
    }

    pub fn get_db_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }
}
