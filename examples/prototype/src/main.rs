use minstant;
use std::time::UNIX_EPOCH;
use chrono::{DateTime, Utc};
use fastdate;

fn main() {
    let anchor = minstant::Anchor::new();
    let instant = minstant::Instant::now();
    let unix_nano = instant.as_unix_nanos(&anchor);
    let minstant_now = DateTime::<Utc>::from_timestamp_nanos(unix_nano as i64);

    let chrono_now = Utc::now();
    let fastdate_now = fastdate::DateTime::now();
    // Print the human-readable date and time
    println!("Minstant datetime: {}", minstant_now);
    println!("Chrono datetime: {}", chrono_now);
    println!("Fastdate datetime: {}", fastdate_now);
}
