use chrono::{DateTime, Utc};

pub fn unix_secs_to_iso(secs: u64) -> String {
    let dt = DateTime::<Utc>::from_timestamp(secs as i64, 0)
        .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());
    dt.to_rfc3339()
}
