use shared_lib::config::env::Environment;

use crate::config::AppConfig;

pub fn get_test_config() -> AppConfig {
    AppConfig {
        environment: Environment::Local,
        database_url: "mysql://root:clearstats-pw@localhost:3307/clearstats".into(),
        clearstats_lab_url: "http://localhost:5010".into(),
        jwt_token_secret: "access-token-secret".into(),
        jwt_token_lifetime: 86400,
        jwt_refresh_token_secret: "refresh-token-secret".into(),
        jwt_refresh_token_lifetime: 864000,
        gcp_project_id: "not-a-project".into(),
        mailersend_api_key: "not-a-real-api-key".into(),
        sentry_dsn_url: None,
    }
}
