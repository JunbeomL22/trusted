use quantlib::data::value_data::ValueData;
use quantlib::currency::Currency;
use time::Instant;
use anyhow::{Result, Context};
use tracing::{info, Level, span};
use serde_json::to_string;
use std::fs::write;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_appender::{rolling, non_blocking};
use time::{Date, Month, Time, UtcOffset, OffsetDateTime};

fn main() -> Result<()> {
    let start_time = Instant::now();
    let file_appender = rolling::daily("logs", "my_app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout);
    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender);
    let subscriber = tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");

    let elapsed = start_time.elapsed();
    
    let value_data = ValueData::new(
        350.0,
        Some(OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2024, Month::January, 1)?,
            Time::from_hms_nano(12, 30, 30, 500_000_000)?,
            UtcOffset::from_hms(9, 0, 0)?,
        )),
        Currency::KRW,
        "KOSPI2".to_string(),
        "KOSPI2".to_string(),
    ).context("Failed to create ValueData")?;

    let json = to_string(&value_data)
        .context("Failed to serialize ValueData to JSON")?;
    write("valuedata.json", &json)
        .context("Failed to write JSON to file")?;



    info!("DataIo finished {:?}", elapsed);
    Ok(())
}