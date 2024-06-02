use quantlib::data::value_data::ValueData;
use quantlib::currency::Currency;
use quantlib::utils::tracing_timer::CustomOffsetTime;
use time::Instant;
use anyhow::{Result, Context};
use tracing::{info, Level, span};
use serde::Deserialize;
use serde_json::{
    to_string,
    from_str,
};
use std::fs::write;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;
use tracing_appender::{rolling, non_blocking};
use time::{Date, Month, Time, UtcOffset, OffsetDateTime};
use examples_toymodel::valuedata_io::valuedata_io;

fn main() -> Result<()> {
    let start_time = Instant::now();
    let file_appender = rolling::daily("logs", "my_app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let custom_time = CustomOffsetTime::new(9, 0, 0);
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_timer(custom_time.clone());

    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_timer(custom_time);
    let subscriber = tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer);
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");

    let elapsed = start_time.elapsed();
    
    valuedata_io()?;

    info!("DataIo finished {:?}", elapsed);
    Ok(())
}
