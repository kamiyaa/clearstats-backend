use chrono::{DateTime, Utc};
use shared_lib::database::DatabaseInteger;

pub fn unix_secs_to_iso(secs: DatabaseInteger) -> String {
    let dt = DateTime::<Utc>::from_timestamp(secs as i64, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());
    dt.to_rfc3339()
}
