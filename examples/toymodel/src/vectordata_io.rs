use anyhow::{Context, Result};
use ndarray::array;
use quantlib::currency::Currency;
use quantlib::data::vector_data::VectorData;
use serde_json::{from_str, to_string_pretty};
use std::fs::write;
use time::macros::datetime;
use tracing::info;

pub fn vectordata_io() -> Result<()> {
    let dates = vec![
        datetime!(2021-01-01 00:00:00 +09:00),
        datetime!(2021-01-03 00:00:00 +09:00),
        datetime!(2021-01-06 00:00:00 +09:00),
        datetime!(2021-01-08 00:00:00 +09:00),
        datetime!(2021-01-11 00:00:00 +09:00),
    ];

    let times = None;
    let values = array![0.1, 0.2, 0.3, 0.4, 0.5];
    let data1 = VectorData::new(
        values,
        Some(dates),
        times,
        Some(datetime!(2021-01-01 17:30:00 +09:00)),
        Currency::KRW,
        "vector_data_name1".to_string(),
        "vector_data_code1".to_string(),
    )
    .expect("Failed to create VectorData1");

    let dates = None;
    let times = Some(array![1.0, 2.0, 3.0, 4.0, 5.0]);
    let values = array![0.1, 0.2, 0.3, 0.4, 0.5];
    let data2 = VectorData::new(
        values,
        dates,
        times,
        None,
        Currency::KRW,
        "vector_data_name2".to_string(),
        "vector_data_code2".to_string(),
    )
    .expect("Failed to create VectorData2");

    let vec_datas_vec = vec![data1, data2];

    let json =
        to_string_pretty(&vec_datas_vec).context("Failed to serialize Vec<VectorData> to JSON")?;
    write("./examples/toymodel/json_data/vectordata.json", json)
        .context("Failed to write JSON to file")?;

    // re-read the file
    let json = std::fs::read_to_string("./examples/toymodel/json_data/vectordata.json")
        .context("Failed to read JSON from file")?;

    let res: Vec<VectorData> =
        from_str(&json).context("Failed to deserialize JSON to VectorData")?;

    info!("{:?}", res);

    Ok(())
}
