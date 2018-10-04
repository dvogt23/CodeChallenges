extern crate chrono;
use chrono::{DateTime, Utc, TimeZone};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let res = (start.timestamp()) + 1000_000_000;
    Utc.timestamp(res, 0)
}
