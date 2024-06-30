use std::fs::write;
use anyhow::{Result, Context};
use time::{Date, Time, UtcOffset, OffsetDateTime, Month};
use serde_json::{
    from_str,
    to_string_pretty,
};
use tracing::info;
use quantlib::data::value_data::ValueData;
use quantlib::currency::Currency;

pub fn valuedata_io() -> Result<()> {
    let kospi_value_data = ValueData::new(
        350.0,
        Some(OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2024, Month::January, 31)?,
            Time::from_hms_nano(7, 13, 31, 874)?,
            UtcOffset::from_hms(9, 0, 0)?,
        )),
        Currency::KRW,
        "KOSPI2".to_string(),
        "KOSPI2".to_string(),
    ).context("Failed to create ValueData")?;

    let spx_value_data = ValueData::new(
        5_000.0,
        Some(OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2024, Month::April, 1)?,
            Time::from_hms_nano(7, 13, 31, 874)?,
            UtcOffset::from_hms(9, 0, 0)?,
        )),
        Currency::USD,
        "SPX".to_string(),
        "SPX".to_string(),
    ).context("Failed to create ValueData")?;

    let value_data_vec = vec![kospi_value_data, spx_value_data];

    let json = to_string_pretty(&value_data_vec)
        .context("Failed to serialize Vec<ValueData> to JSON")?;
    write("./examples/toymodel/json_data/valuedata.json", json).context("Failed to write JSON to file")?;
    
    // re-read the file
    let json = std::fs::read_to_string("./examples/toymodel/json_data/valuedata.json")
        .context("Failed to read JSON from file")?;

    let res: Vec<ValueData> = from_str(&json)
        .context("Failed to deserialize JSON to ValueData")?;

    info!("{:?}", res);

    Ok(())
}