use axum::http::StatusCode;
use shared_lib::{
    config::{env::Environment, service_config::ServiceConfig},
    error::{AppServerResult, ServerErrorResponse},
    utils,
};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub environment: Environment,
    pub database_url: String,
    pub indaggo_lab_url: String,
    pub jwt_token_secret: String,
    pub jwt_token_lifetime: u64,
    pub jwt_refresh_token_secret: String,
    pub jwt_refresh_token_lifetime: u64,
    pub gcp_project_id: String,
    pub mailersend_api_key: String,
    pub sentry_dsn_url: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> AppServerResult<Self> {
        let environment = Environment::from_env_var("ENV")?;
        let database_url = utils::get_env_var("DATABASE_URL")?;

        let indaggo_lab_url = utils::get_env_var("INDAGGO_LAB_URL")?;

        let jwt_token_secret = utils::get_env_var("JWT_TOKEN_SECRET")?;
        let jwt_token_lifetime = utils::get_env_var("JWT_TOKEN_LIFETIME")?;
        let jwt_token_lifetime = jwt_token_lifetime.parse().map_err(|err| {
            let error_msg =
                format!("Failed to parse JWT_TOKEN_LIFETIME env var '{jwt_token_lifetime}': {err}");
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;
        let jwt_refresh_token_secret = utils::get_env_var("JWT_REFRESH_TOKEN_SECRET")?;

        let jwt_refresh_token_lifetime = utils::get_env_var("JWT_REFRESH_TOKEN_LIFETIME")?;
        let jwt_refresh_token_lifetime = jwt_refresh_token_lifetime.parse().map_err(|err| {
            let error_msg = format!(
                "Failed to parse JWT_REFRESH_TOKEN_LIFETIME env var '{jwt_token_lifetime}': {err}"
            );
            ServerErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, 1000, error_msg)
        })?;

        let gcp_project_id = utils::get_env_var("GCP_PROJECT_ID")?;
        let mailersend_api_key = utils::get_env_var("MAILERSEND_API_KEY")?;

        let sentry_dsn_url = utils::get_env_var("SENTRY_DSN_URL").ok();

        Ok(Self {
            environment,
            database_url,
            indaggo_lab_url,
            jwt_token_secret,
            jwt_token_lifetime,
            jwt_refresh_token_secret,
            jwt_refresh_token_lifetime,
            gcp_project_id,
            mailersend_api_key,
            sentry_dsn_url,
        })
    }

    pub fn get_jwt_token_secret(&self) -> &str {
        &self.jwt_token_secret
    }

    pub fn get_jwt_refresh_token_secret(&self) -> &str {
        &self.jwt_refresh_token_secret
    }
}

impl ServiceConfig for AppConfig {
    fn get_environment(&self) -> Environment {
        self.environment
    }
}
