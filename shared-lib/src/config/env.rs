use reqwest::StatusCode;

use crate::{
    error::{AppServerResult, ServerErrorResponse},
    utils,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Environment {
    Production,
    Development,
    Local,
}

impl Environment {
    pub fn from_env_var(s: &str) -> AppServerResult<Self> {
        let value = utils::get_env_var(s)?;
        match value.as_str() {
            "prod" => Ok(Self::Production),
            "dev" => Ok(Self::Development),
            "local" => Ok(Self::Local),
            s => Err(ServerErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                1000,
                format!("Unknown environment {s}"),
            )),
        }
    }
}
