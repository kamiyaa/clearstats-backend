use axum::http::StatusCode;
use chrono::{DateTime, Utc};

use crate::{
    database::DatabaseInteger,
    error::{AppServerResult, ServerErrorResponse},
};

pub fn get_secs_since_epoch() -> AppServerResult<DatabaseInteger> {
    let now = Utc::now();
    Ok(now.timestamp() as DatabaseInteger)
}

pub fn utc_to_secs(d: &DateTime<Utc>) -> DatabaseInteger {
    d.timestamp() as DatabaseInteger
}

pub fn rfc3339_to_secs(s: &str) -> AppServerResult<DatabaseInteger> {
    let rfc3339 = DateTime::parse_from_rfc3339(s).map_err(|err| {
        tracing::error!("{err}");
        ServerErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            1234,
            "Failed to parse timestamp".to_string(),
        )
    })?;

    Ok(rfc3339.timestamp() as DatabaseInteger)
}
