use crate::currency::Currency;
use crate::definitions::{Real, Time};
use crate::time::{calendars::nullcalendar::NullCalendar, calendar_trait::CalendarTrait};
use time::OffsetDateTime;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

use ndarray::Array1;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

#[derive(Serialize, Deserialize, Clone)]
pub struct VectorData {
    value: Array1<Real>,
    dates: Option<Vec<OffsetDateTime>>,
    times: Array1<Time>,
    market_datetime: Option<OffsetDateTime>,
    currency: Currency,
    name: String,
    code: String,
}

impl fmt::Debug for VectorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VectorData")
            .field("value", &self.value)
            .field("dates", &self.dates)
            .field("times", &self.times)
            .field("market_datetime", &self.market_datetime)
            .field("currency", &self.currency)
            .field("name", &self.name)
            .field("code", &self.code)
            .finish()
    }
}

impl VectorData {
    /// value: Array1<Real>,
    /// dates: Option<Vec<OffsetDateTime>>,
    /// times: Option<Array1<Time>>,
    /// market_datetime: OffsetDateTime,
    /// name: String
    pub fn new(
        value: Array1<Real>, 
        dates: Option<Vec<OffsetDateTime>>, 
        times: Option<Array1<Time>>,
        market_datetime: Option<OffsetDateTime>, 
        currency: Currency,
        name: String,
        code: String,
    ) -> Result<VectorData> {
        // sanity check first
        if dates == None && times == None {
            return Err(anyhow!(
                "dates and times are both None"
            ));
        }

        if let Some(dates) = &dates {
            // change the following assertion to return Err
            if value.len() != dates.len() {
                return Err(anyhow!(
                    "The length of value and dates must be the same\n\
                    value: {:?}, dates: {:?}",
                    value,
                    dates,
                ));
            }
            
            let market_datetime = match market_datetime {
                Some(market_datetime) => market_datetime,
                None => {
                    return Err(anyhow!(
                        "({}:{}) the dates in VectorData of {} is not None, but market_datetime is None\n\
                        Thus, it is vague to calculate the time difference between market_datetime and dates",
                        file!(), line!(), name
                    ));
                }
            };
            let time_calculator = NullCalendar::default();
            let times: Array1<Time> = dates
            .iter()
            .map(|date| time_calculator.get_time_difference(&market_datetime, date))
            .collect();
            
            let res = VectorData {
                value,
                dates: Some(dates.to_vec()),
                times: times,
                market_datetime: Some(market_datetime),
                currency: currency,
                name: name,
                code: code,
            };
            Ok(res)
        } else {
            if let Some(times) = times {
                if value.len() != times.len() {
                    return Err(anyhow!(
                        "The length of value and times must be the same\n\
                        value: {:?}, times: {:?}",
                        value,
                        times,
                    ));
                } else {
                    let res = VectorData {
                        value,
                        dates,
                        times: times,
                        market_datetime,
                        currency: currency,
                        name,
                        code,
                    };
                    Ok(res)
                }
            } else {
                return Err(anyhow!("dates and times are both None"));
            }
        }
    }

    pub fn get_name_clone(&self) -> String {
        self.name.clone()
    }

    pub fn get_value_clone(&self) -> Array1<Real> {
        self.value.clone()
    }

    pub fn get_times_clone(&self) -> Array1<Time> {
        self.times.clone()
    }

    pub fn get_dates_clone(&self) -> Option<Vec<OffsetDateTime>> {
        self.dates.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use time::macros::datetime;
    use ndarray::array;

    #[test]
    fn test_vector_data_serialization() {
        let vector_data = VectorData::new(
            array![1.0, 2.0, 3.0, 4.0, 5.0], 
            None, 
            Some(array![0.0, 1.0, 2.0, 3.0, 4.0]), 
            None,//datetime!(2020-01-01 00:00:00 UTC), 
            Currency::KRW,
            "test_vector_data_serialization".to_string(),
            "test_vector_data_serialization".to_string()
        ).expect("failed to create VectorData");

        let serialized = serde_json::to_string(&vector_data).unwrap();
        println!("VectorData serialized = {}", serialized);
        let desrialized: VectorData = serde_json::from_str(&serialized).unwrap();
        println!("VectorData deserialized = {:?}", desrialized);
        
        // value check
        assert_eq!(vector_data.get_value_clone(), desrialized.get_value_clone());
        // times check
        assert_eq!(vector_data.get_times_clone(), desrialized.get_times_clone());
    }
} 
