use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use time::{Date, Time, OffsetDateTime, UtcOffset};
use crate::data::daily_value_data::DailyValueData;
use crate::definitions::Real;
use crate::time::calendar::Calendar;
use crate::time::calendar_trait::CalendarTrait;
use crate::time::calendars::southkorea::{
    SouthKorea,
    SouthKoreaType,
};
use crate::time::conventions::BusinessDayConvention;
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyClosePrice {
    value: HashMap<Date, Real>,
    close_time: Time,
    utc_offset: UtcOffset,
    calendar: Calendar,
    name: String,
    code: String,
}

impl Default for DailyClosePrice {
    fn default() -> Self {
        DailyClosePrice {
            value: HashMap::new(),
            // korea stock market
            close_time: Time::from_hms(15, 40, 0).unwrap(), 
            utc_offset: UtcOffset::from_hms(9, 0, 0).unwrap(), 
            calendar: Calendar::SouthKorea(
                SouthKorea::new(SouthKoreaType::Krx),
            ),
            name: String::new(),
            code: String::new(),
        }
    }
}

impl DailyClosePrice {
    pub fn new(
        value: HashMap<Date, Real>, 
        close_time: Time,
        utc_offset: UtcOffset,
        calendar: Calendar,
        name: String, 
        code: String,
    ) -> DailyClosePrice {
        DailyClosePrice {
            value,
            close_time,
            utc_offset,
            calendar,
            name,
            code,
        }
    }

    pub fn new_from_data(
        data: &DailyValueData,
    ) -> Result<DailyClosePrice> {
        let res = DailyClosePrice {
            value: data.get_value().clone(),
            close_time: *data.get_close_time(),
            utc_offset: *data.get_utc_offset(),
            calendar: data.get_calendar().clone(),
            name: data.get_name().clone(),
            code: data.get_code().clone(),
        };
        Ok(res)
    }

    pub fn get_value(&self) -> &HashMap<Date, Real> {
        &self.value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_code(&self) -> &String {
        &self.code
    }

    // Get method
    pub fn get(&self, key: &Date) -> Option<&Real> {
        self.value.get(key)
    }

    /// input datetime: OffsetDateTime
    /// if the datetime is after the close time considering the offset, return the next date's close price
    pub fn get_at_datetime(&self, datetime: &OffsetDateTime) -> Option<&Real> {
        let datetime = datetime.to_offset(self.utc_offset);

        if datetime.time() > self.close_time {
            let next_date = self.calendar.adjust(
                &(datetime + time::Duration::days(1)), 
                &BusinessDayConvention::Following,
            ).expect("Failed to adjust date in get_at_datetime");
            self.value.get(&next_date.date())
        } else {
            self.value.get(&datetime.date())
        }
    }

    pub fn get_close_time(&self) -> &Time {
        &self.close_time
    }

    pub fn get_utc_offset(&self) -> &UtcOffset {
        &self.utc_offset
    }

    // Get mutable method
    pub fn get_mut(&mut self, key: &Date) -> Option<&mut Real> {
        self.value.get_mut(key)
    }

    pub fn set_value(&mut self, value: HashMap<Date, Real>) {
        self.value = value;
    }

    pub fn get_ordered_data_by_datetime(&self) -> (Vec<Date>, Vec<Real>) {
        let mut ordered_data = self.value.iter().collect::<Vec<_>>();
        ordered_data.sort_by(|a, b| a.0.cmp(b.0));
        let mut ordered_datetime = Vec::new();
        let mut ordered_value = Vec::new();
        for (datetime, value) in ordered_data {
            ordered_datetime.push(*datetime);
            ordered_value.push(*value);
        }
        (ordered_datetime, ordered_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn test_close_data() {
        let mut past_price = DailyClosePrice::default();
        let date1 = date!(2021-01-01);
        let date2 = date!(2021-01-02);
        let date3 = date!(2021-01-03);

        let value_map = vec![(date1, 100.0), (date2, 200.0), (date3, 300.0)]
            .into_iter()
            .collect::<HashMap<Date, Real>>();
        
        past_price.set_value(value_map);

        assert_eq!(past_price.get(&date1), Some(&100.0));
        assert_eq!(past_price.get(&date2), Some(&200.0));
        assert_eq!(past_price.get(&date3), Some(&300.0));
        let (ordered_datetime, ordered_value) = past_price.get_ordered_data_by_datetime();
        assert_eq!(ordered_datetime, vec![date1, date2, date3]);
        assert_eq!(ordered_value, vec![100.0, 200.0, 300.0]);
    }
}