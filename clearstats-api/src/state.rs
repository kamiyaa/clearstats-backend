use shared_lib::database::manager::DatabaseManager;

use crate::config::AppConfig;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_manager: DatabaseManager,
    pub config: AppConfig,
}

impl AppState {
    pub fn get_db_manager(&self) -> &DatabaseManager {
        &self.db_manager
    }
}
