use crate::time::calendar_trait::CalendarTrait;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NullCalendar {
    name: String,
}

impl Default for NullCalendar {
    fn default() -> NullCalendar {
        NullCalendar {
            name: "NullCalendar".to_string(),
        }
    }
}

impl NullCalendar {
    pub fn new() -> NullCalendar {
        NullCalendar::default()
    }
}

impl CalendarTrait for NullCalendar {
    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        self._is_weekend(date)
    }

    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        self._is_holiday(date)
    }

    fn is_removed_holiday(&self, _date: &OffsetDateTime) -> bool {
        false
    }

    fn is_added_holiday(&self, _date: &OffsetDateTime) -> bool {
        false
    }

    fn is_base_holiday(&self, _date: &OffsetDateTime) -> bool {
        false
    }

    fn calendar_name(&self) -> &String {
        &self.name
    }

    fn add_holidays(&mut self, _date: &Date) -> Result<()> {
        Ok(())
    }

    fn remove_holidays(&mut self, _date: &Date) -> Result<()> {
        Ok(())
    }

    fn display_holidays(
        &self,
        date_from: &OffsetDateTime,
        date_upto: &OffsetDateTime,
        include_weekend: bool,
    ) {
        self._display_holidays(date_from, date_upto, include_weekend);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::Time;
    use crate::time::calendars::nullcalendar::NullCalendar;
    use crate::time::conventions::DayCountConvention;
    use crate::utils::string_arithmetic::add_period;
    use time::macros::{date, datetime};
    use time::Month;

    #[test]
    fn test_is_weekend() {
        let calendar = NullCalendar::default();
        let weekday = datetime!(2021-12-3 0:0:0 UTC); // Friday, 3rd December 2021
        let weekend = datetime!(2021-12-4 0:0:0 UTC); // Saturday, 4th December 2021

        assert_eq!(calendar.is_weekend(&weekday), false);
        assert_eq!(calendar.is_weekend(&weekend), true);
    }

    #[test]
    fn test_is_holiday() {
        let calendar = NullCalendar::default();
        let date = datetime!(2021-12-3 0:0:0 UTC); // Friday, 3rd December 2021

        assert_eq!(calendar.is_holiday(&date), false);
    }

    #[test]
    fn test_is_leap_year() {
        let calendar = NullCalendar::default();

        assert_eq!(calendar.is_leap_year(2000), true); // 2000 is a leap year
        assert_eq!(calendar.is_leap_year(2021), false); // 2021 is not a leap year
    }

    #[test]
    fn test_last_day_of_month() {
        let calendar = NullCalendar::default();

        assert_eq!(
            calendar.last_day_of_month(2021, Month::February),
            date!(2021 - 02 - 28)
        ); // 2021 is not a leap year
        assert_eq!(
            calendar.last_day_of_month(2020, Month::February),
            date!(2020 - 02 - 29)
        ); // 2020 is a leap year
        assert_eq!(
            calendar.last_day_of_month(2021, Month::December),
            date!(2021 - 12 - 31)
        ); // December always has 31 days
    }

    #[test]
    fn test_get_time_difference() {
        let calendar = NullCalendar::default();

        // one year
        let start_date = datetime!(2021-12-3 00:00:00 UTC); // Friday, 3rd December 2021
        let end_date = datetime!(2021-12-4 00:00:00 UTC); // Saturday, 4th December 2021
        let res = calendar.get_time_difference(&start_date, &end_date);
        assert!(
            (res - 1.0 / 365.0).abs() < 1e-5,
            "calculated time difference from {} to {} = {}, expected = {}",
            start_date,
            end_date,
            res,
            1.0 / 365.0
        );

        // same datetime but with different offset
        let start_date = datetime!(2021-12-3 00:00:00 UTC); // Friday, 3rd December 2021
        let end_date = datetime!(2021-12-3 00:00:00 -12:00); // Friday, 3rd December 2021
        let res = calendar.get_time_difference(&start_date, &end_date);
        assert!(
            (res - 0.5 / 365.0).abs() < 1e-5,
            "calculated time difference from {} to {} = {}, expected = {}",
            start_date,
            end_date,
            res,
            0.5 / 365.0
        );

        // one year difference where the start_date is leap_year with different offset
        let start_date = datetime!(2020-01-01 00:00:00 UTC);
        let end_date = datetime!(2021-01-01 00:00:00 -06:00);
        let res = calendar.get_time_difference(&start_date, &end_date);
        let expected: Time = 1.0 + 0.25 / 365.;
        assert!(
            (res - expected).abs() < 1e-5,
            "calculated time difference from {} to {} = {}, expected = {}",
            start_date,
            end_date,
            res,
            expected
        );

        // one year and one day difference where the start_date is leap_year with different offset
        let start_date = datetime!(2020-01-02 00:00:00 UTC);
        let end_date = datetime!(2021-01-01 00:00:00 -06:00);
        let res = calendar.get_time_difference(&start_date, &end_date);
        let expected: Time = 1.0 + 0.25 / 365. - 1.0 / 366.; // leap_year
        assert!(
            (res - expected).abs() < 1e-5,
            "calculated time difference from {} to {} = {}, expected = {}",
            start_date,
            end_date,
            res,
            expected
        );

        // one year and one day difference where the start_date is leap_year with different offset
        let start_date = datetime!(2020-01-01 00:00:00 UTC);
        let end_date = datetime!(2021-01-02 00:00:00 -06:00);
        let res = calendar.get_time_difference(&start_date, &end_date);
        let expected: Time = 1.0 + 0.25 / 365. + 1.0 / 365.; // leap_year
        assert!(
            (res - expected).abs() < 1e-5,
            "calculated time difference from {} to {} = {}, expected = {}",
            start_date,
            end_date,
            res,
            expected
        );
    }

    #[test]
    fn test_act_act_isda() {
        let calendar = NullCalendar::default();
        let start_date = datetime!(2021-01-01 0:0:0 UTC);

        for i in 1..=100 {
            let end_date = add_period(&start_date, &format!("{}Y", i));
            let result = calendar
                .year_fraction(&start_date, &end_date, &DayCountConvention::ActActIsda)
                .expect("Failed to calculate year fraction");
            assert!(
                (result - i as Time).abs() < 1e-5,
                "calculated year_fraction from {} to {} = {}, expected = {}",
                start_date,
                end_date,
                result,
                i
            );
        }
    }
}
