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
    //
    let unix_nano_string = format!("{}", unix_nano);
    let unix_nano_byte1 = unix_nano_string.as_bytes();
    let unix_nano_byte2 = unix_nano.to_ne_bytes();
    let unix_nano_byte3 = unix_nano.to_be_bytes();

    println!("unix_nano_format_byte: {:?}", unix_nano_byte1);
    println!("unix_nano_ne_byte: {:?}", unix_nano_byte2);
    println!("unix_nano_be_byte: {:?}", unix_nano_byte3);
}
