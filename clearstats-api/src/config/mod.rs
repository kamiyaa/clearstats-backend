use shared_lib::{
    config::{env::Environment, service_config::ServiceConfig},
    error::{AppServerResult},
    utils,
};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub environment: Environment,
    pub database_url: String,
    pub jwt_token_secret: String,
    pub sentry_dsn_url: Option<String>,
}

impl AppConfig {
    pub fn from_env() -> AppServerResult<Self> {
        let environment = Environment::from_env_var("ENV")?;
        let database_url = utils::get_env_var("DATABASE_URL")?;

        let jwt_token_secret = utils::get_env_var("JWT_TOKEN_SECRET")?;
        let sentry_dsn_url = utils::get_env_var("SENTRY_DSN_URL").ok();

        Ok(Self {
            environment,
            database_url,
            jwt_token_secret,
            sentry_dsn_url,
        })
    }

    pub fn get_jwt_token_secret(&self) -> &[u8] {
        self.jwt_token_secret.as_bytes()
    }
}

impl ServiceConfig for AppConfig {
    fn get_environment(&self) -> Environment {
        self.environment
    }
}
