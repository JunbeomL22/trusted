use time::{Date, Month, Weekday, OffsetDateTime};
use crate::time::conventions::BusinessDayConvention;
use crate::time::conventions::DayCountConvention;
use crate::definitions::Time;
use crate::time::holiday::Holidays;
use serde::{Deserialize, Serialize};
use crate::utils::myerror::MyError;
use anyhow::Result;

pub enum Calendar {
    NullCalendar(Box<dyn CalendarTrait>),
    SouthKorea(Box<dyn CalendarTrait>),
    UnitedStates(Box<dyn CalendarTrait>),
    JointCalendar(Box<dyn CalendarTrait>),
}

impl Calendar {
    pub fn as_trait(&self) -> &(dyn CalendarTrait) {
        match self {
            Calendar::NullCalendar(cal) => &**cal,
            Calendar::SouthKorea(cal) => &**cal,
            Calendar::UnitedStates(cal) => &**cal,
            Calendar::JointCalendar(cal) => &**cal,
        }
    }
}

pub trait CalendarTrait {
    fn unpack_date(&self, date: &OffsetDateTime) -> (i32, Month, u8, Weekday, u16) {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let weekday = date.weekday();
        let day_of_year = date.ordinal();

        (year, month, day, weekday, day_of_year)
    }  

    fn _is_weekend(&self, date: &OffsetDateTime) -> bool {
        let w = date.weekday();
        w == Weekday::Saturday || w == Weekday::Sunday
    }
    fn is_weekend(&self, date: &OffsetDateTime) -> bool;
    
    fn display_holidays(&self, 
                        date_from: &OffsetDateTime, 
                        date_upto: &OffsetDateTime,
                        include_weekend: bool);

    fn _display_holidays(&self, 
                        date_from: &OffsetDateTime, 
                        date_upto: &OffsetDateTime,
                        include_weekend: bool) {
        let mut date = *date_from;
        while date <= *date_upto {
            if self.is_holiday(&date) && (include_weekend || !self.is_weekend(&date)) {
                println!("{:?}", date);
            }
            date = date + time::Duration::days(1);
        }
    }

    fn is_removed_holiday(&self, date: &OffsetDateTime) -> bool;
    fn is_added_holiday(&self, date: &OffsetDateTime) -> bool;
    fn is_base_holiday(&self, date: &OffsetDateTime) -> bool;

    fn is_business_day(&self, date: &OffsetDateTime) -> bool {
        !self.is_holiday(date)
    }

    fn calendar_name(&self) -> &String;
    fn add_holidays(&mut self, date: &Date) -> Result<(), MyError>;
    fn remove_holidays(&mut self, date: &Date) -> Result<(), MyError>;

    fn is_holiday(&self, date: &OffsetDateTime) -> bool;
    fn _is_holiday(&self, date: &OffsetDateTime) -> bool {
        // check order is as follows:
        // weekend => removed holiday => base holiday following specific calendar => added holiday
        // 1) weekend
        if self.is_weekend(date) {
            return true;
        }
        // 2) removed holiday
        if self.is_removed_holiday(date) {
            return false;
        }
        // 3) base holidays following to specific calendar
        if self.is_base_holiday(date) {
            return true;
        }
        // 4) added holiday
        if self.is_added_holiday(date) {
            return true;
        }
        return false;
    }

    fn adjust_following(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let mut res = date.clone();

        while self.is_holiday(&res) {
            res = res + time::Duration::days(1);
        }
        res
    }

    fn adjust_preceding(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let mut res = date.clone();

        while self.is_holiday(&res) {
            res = res - time::Duration::days(1);
        }
        res
    }

    fn adjust_modified_following(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let month = date.month();

        let mut res = date.clone();

        while self.is_holiday(&res) {
            res = res + time::Duration::days(1);
        }

        if month != res.month() {
            res = date.clone();
            self.adjust_preceding(&res)    
        }
        else {
            res
        }
    }

    fn adjust_modified_preceding(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let month = date.month();

        let mut res = date.clone();

        while self.is_holiday(&res) {
            res = res - time::Duration::days(1);
        }

        if month != res.month() {
            res = date.clone();
            self.adjust_following(&res)    
        }
        else {
            res
        }
    }

    fn adjust(&self, date: &OffsetDateTime, convention: &BusinessDayConvention) -> OffsetDateTime {
        match convention {
            BusinessDayConvention::Unadjusted => *date,
            BusinessDayConvention::Following => self.adjust_following(&date),
            BusinessDayConvention::Preceding => self.adjust_preceding(&date),
            BusinessDayConvention::ModifiedPreceding => self.adjust_modified_preceding(&date),
            BusinessDayConvention::ModifiedFollowing => self.adjust_modified_following(&date),
        }
    }

    /// This year fraction is calculated mostly for coupon amount
    /// Thus, the functino considers only the days between date_from and date_upto
    /// For the exact time, use get_time_difference function (calculated by ActACtIsda Fasion)
    fn year_fraction(
            &self, 
            start_date: &OffsetDateTime, 
            end_date: &OffsetDateTime, 
            day_count: &DayCountConvention) -> Time {
        // sanity check
        assert!(
            start_date <= end_date,
            "start_date should be less than or equal to end_date: start_date: {:?}, end_date: {:?}",
            start_date,
            end_date
        );
        let res = match day_count {
            DayCountConvention::Actual365Fixed => {
                let days = *end_date - *start_date;
                days.whole_days() as Time / 365.0
                },
            DayCountConvention::Actual360 => {
                let days = *end_date - *start_date;
                days.whole_days() as Time / 360.0
                },
            DayCountConvention::Actual364 => {
                let days = *end_date - *start_date;
                days.whole_days() as Time / 364.0
                },
            DayCountConvention::Thirty360 => {
                let (year_from, month_from, day_from, _, _) = self.unpack_date(start_date);
                let (year_upto, month_upto, day_upto, _, _) = self.unpack_date(end_date);
                let mut days = 0;
                days += 360 * (year_upto - year_from);
                days += 30 * (month_upto as i32 - month_from as i32);
                days += day_upto as i32 - day_from as i32;
                days as Time / 360.0
                },
            //Each month is treated normally and the year is assumed to be 365 days. 
            //For example, in a period from February 1, 2005, to April 1, 2005, the Factor is considered to be 59 days divided by 365.
            //The CouponFactor uses the same formula, replacing Date2 by Date3. 
            //In general, coupon payments will vary from period to period, due to the differing number of days in the periods. 
            //The formula applies to both regular and irregular coupon periods.
            //Reference: https://www.isda.org/a/7jEEA/2006-ISDA-Definitions-Section-4.16.pdf
            DayCountConvention::ActActIsda => {
                let mut frac: Time = 0.0;
                let start_year = start_date.year();
                let end_year = end_date.year();

                for year in start_year..=end_year {
                    let days_in_year = if self.is_leap_year(year) { 366.0 } else { 365.0 };

                    if year == start_year {
                        let days = if end_year == start_year {
                            (*end_date - *start_date).whole_days() as Time
                        } else {
                            let last_day_of_year = Date::from_calendar_date(year, Month::December, 31).unwrap();
                            (last_day_of_year - start_date.date()).whole_days() as Time
                        };
                        frac += days / days_in_year;
                    } 
                    else if year == end_year {
                        let last_day_of_previous_year = Date::from_calendar_date(year - 1 , Month::December, 31).unwrap();
                        let days = (end_date.date() - last_day_of_previous_year).whole_days() as Time;
                        frac += days / days_in_year;
                    } 
                    else {
                        frac += 1.0; // Full year
                    };
                }
                frac
                },
            };
        res
    }

    /// The last calendar day of a month, not the last business day
    fn last_day_of_month(&self, year: i32, month: Month) -> Date {
        let last_day = match month {
            Month::January | Month::March | Month::May | Month::July | Month::August | Month::October | Month::December => {
                Date::from_calendar_date(year, month, 31).unwrap()
            },

            Month::April | Month::June | Month::September | Month::November => {
                Date::from_calendar_date(year, month, 30).unwrap()
            },
            Month::February => {
                if self.is_leap_year(year) {
                    Date::from_calendar_date(year, month, 29).unwrap()
                } else {
                    Date::from_calendar_date(year, month, 28).unwrap()
                }
            },
        };
        last_day
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// returns the time in yearly fraction
    /// If start_date and end_date is not in the leap year, then the time is calculated as Act365 fasion
    /// If start_date is in the leap year and end_date is not in the leap year, 
    /// then, at the first, calculate the time from start_date to the end of the leap year as Act366 fasion
    /// at the second, calculate the time from the start of the end_date to the end_date as Act365 fasion
    /// then, sum up the two times
    /// If start_date is not in leap year and end_date is in the leap year,
    /// then, calculate the time from start_date to the end of year of start_date as Act365 fasion
    /// at the second, calculate the time from the start of the end_date to the end_date as Act366 fasion
    /// then, sum up the two times
    fn get_time_difference(&self, start_date: &OffsetDateTime, end_date: &OffsetDateTime) -> Time {
        //let midnight_last_day_of_year_start = OffsetDateTime::new_in_offset(last_day_of_year_start, time::Time::MIDNIGHT, start_date_offset);
        let mut frac: Time = 0.0;
        let start_year = start_date.year();
        let end_year = end_date.year();

        for year in start_year..=end_year {
            let days_in_year: Time = if self.is_leap_year(year) { 366.0 } else { 365.0 };
            let start_date_offset = start_date.offset();
            if year == start_year {
                let days = if end_year == start_year {
                    (*end_date - *start_date).as_seconds_f64() as Time / (24.0 * 60.0 * 60.0)
                } else {
                    let last_day_of_year_start = Date::from_calendar_date(year, Month::December, 31).unwrap();
                    let last_day_of_year = OffsetDateTime::new_in_offset(
                                                                    last_day_of_year_start, 
                                                                    time::Time::MIDNIGHT, 
                                                                    start_date_offset
                                                                );
                    (last_day_of_year - *start_date).as_seconds_f64() as Time / (24.0 * 60.0 * 60.0)
                };
                
                frac += days as Time / days_in_year;
            } 
            else if year == end_year {
                let _last_day_of_previous_year = Date::from_calendar_date(year - 1 , Month::December, 31).unwrap();
                let last_day_of_previous_year = OffsetDateTime::new_in_offset(
                                                                    _last_day_of_previous_year, 
                                                                    time::Time::MIDNIGHT, 
                                                                    start_date_offset
                                                                );
                let days = (*end_date - last_day_of_previous_year).as_seconds_f64() as Time / (24.0 * 60.0 * 60.0);
                frac += days / days_in_year;
            } 
            else {
                frac += 1.0; // Full year
            };
        }
        frac
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct NullCalendar {}

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
        &"NullCalendar".to_string()
    }

    fn add_holidays(&mut self, _date: &Date) -> Result<(), MyError> { Ok(()) }

    fn remove_holidays(&mut self, _date: &Date) -> Result<(), MyError> { Ok(()) }

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
    use time::macros::{date, datetime};
    use crate::utils::string_arithmetic::add_period;

    #[test]
    fn test_is_weekend() {
        let calendar = NullCalendar {};
        let weekday = datetime!(2021-12-3 0:0:0 UTC); // Friday, 3rd December 2021
        let weekend = datetime!(2021-12-4 0:0:0 UTC); // Saturday, 4th December 2021

        assert_eq!(calendar.is_weekend(&weekday), false);
        assert_eq!(calendar.is_weekend(&weekend), true);
    }

    #[test]
    fn test_is_holiday() {
        let calendar = NullCalendarType {};
        let date = datetime!(2021-12-3 0:0:0 UTC); // Friday, 3rd December 2021

        assert_eq!(calendar.is_holiday(&date), false);
    }

    #[test]
    fn test_is_leap_year() {
        let calendar = NullCalendar {};

        assert_eq!(calendar.is_leap_year(2000), true); // 2000 is a leap year
        assert_eq!(calendar.is_leap_year(2021), false); // 2021 is not a leap year
    }

    #[test]
    fn test_last_day_of_month() {
        let calendar = NullCalendar {};

        assert_eq!(calendar.last_day_of_month(2021, Month::February), date!(2021-02-28)); // 2021 is not a leap year
        assert_eq!(calendar.last_day_of_month(2020, Month::February), date!(2020-02-29)); // 2020 is a leap year
        assert_eq!(calendar.last_day_of_month(2021, Month::December), date!(2021-12-31)); // December always has 31 days
    }

    #[test]
    fn test_get_time_difference() {
        let calendar = NullCalendar {};

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
        let calendar = NullCalendar {};
        let start_date = datetime!(2021-01-01 0:0:0 UTC);
        
        for i in 1..=100 {
            let end_date = add_period(&start_date, &format!("{}Y", i));
            let result = calendar.year_fraction(&start_date, &end_date, &DayCountConvention::ActActIsda);
            assert!((result - i as Time).abs() < 1e-5,
                "calculated year_fraction from {} to {} = {}, expected = {}",
                start_date,
                end_date,
                result,
                i
            );
        }
    }
}