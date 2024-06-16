use minstant;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, Utc};
use time::ext;

fn main() {
    let anchor = minstant::Anchor::new();
    let instant = minstant::Instant::now();
    let unix_nano = instant.as_unix_nanos(&anchor);
    let datetime = DateTime::<Utc>::from_timestamp_nanos(unix_nano as i64);

    // Print the human-readable date and time
    println!("Human-readable datetime: {}", datetime);
}
