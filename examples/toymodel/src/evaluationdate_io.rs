use quantlib::evaluation_date::EvaluationDate;
use time::macros::datetime;
use anyhow::{Result, Context};
use serde_json::{
    from_str,
    to_string,
};
use tracing::info;
use std::fs::{
    write,
    read_to_string,
};

pub fn evaluationdate_io() -> Result<()> {
    let evaluation_datetime = datetime!(2021-01-01 17:00:00 +09:00);
    let data1 = EvaluationDate::new(evaluation_datetime);
    let json = to_string(&data1)
        .context("Failed to serialize EvaluationDate to JSON")?;
    write("json_data/evaluationdate.json", &json)
        .context("Failed to write JSON to file")?;

    let json = read_to_string("json_data/evaluationdate.json")
        .context("Failed to read JSON from file")?;

    let res: EvaluationDate = from_str(&json)
        .context("Failed to deserialize JSON to EvaluationDate")?;

    info!("{:?}", res);
    Ok(())
}