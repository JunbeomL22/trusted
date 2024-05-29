use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use time::{Date, Time, UtcOffset};
use crate::definitions::Real;
use crate::time::calendar::Calendar;
use crate::time::calendars::southkorea::{
    SouthKorea,
    SouthKoreaType,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyValueData {
    value: HashMap<Date, Real>,
    close_time: Time,
    utc_offset: UtcOffset,
    calendar: Calendar,
    name: String,
    code: String,
}

impl Default for DailyValueData {
    fn default() -> Self {
        DailyValueData {
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

impl DailyValueData {
    pub fn new(
        value: HashMap<Date, Real>, 
        close_time: Time,
        utc_offset: UtcOffset,
        calendar: Calendar,
        name: String, 
        code: String,
    ) -> DailyValueData {
        DailyValueData {
            value,
            close_time,
            utc_offset,
            calendar,
            name,
            code,
        }
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

    pub fn get_calendar(&self) -> &Calendar {
        &self.calendar
    }

    pub fn get_close_time(&self) -> &Time {
        &self.close_time
    }

    pub fn get_utc_offset(&self) -> &UtcOffset {
        &self.utc_offset
    }

    pub fn insert(&mut self, key: Date, value: Real) {
        self.value.insert(key, value);
    }

    // Get mutable method
    pub fn get_mut(&mut self, key: &Date) -> Option<&mut Real> {
        self.value.get_mut(key)
    }

    pub fn get_ordered_data_by_date(&self) -> (Vec<Date>, Vec<Real>) {
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
        let mut data = DailyValueData::default();
        let date1 = date!(2021-01-01);
        let date2 = date!(2021-01-02);
        let date3 = date!(2021-01-03);
        data.insert(date1, 100.0);
        data.insert(date2, 200.0);
        data.insert(date3, 300.0);
        
        assert_eq!(data.get(&date1), Some(&100.0));
        assert_eq!(data.get(&date2), Some(&200.0));
        assert_eq!(data.get(&date3), Some(&300.0));
        let (ordered_datetime, ordered_value) = data.get_ordered_data_by_date();
        assert_eq!(ordered_datetime, vec![date1, date2, date3]);
        assert_eq!(ordered_value, vec![100.0, 200.0, 300.0]);
    }
}