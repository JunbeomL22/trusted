use time::{Date, Month, Weekday, OffsetDateTime};
use crate::time::conventions::BusinessDayConvention;
use crate::time::conventions::DayCountConvention;
use crate::definitions::Real;
use crate::time::constants::{EASTER_MONDAYS, FIRST_EASTER_MONDAY, LAST_EASTER_MONDAY};
use log::warn;
pub trait Holidays {
    #[allow(unused_variables)]
    fn is_last_business_day_of_year(&self, date: &OffsetDateTime) -> bool { return false}
    fn is_holiday(&self, date: &OffsetDateTime) -> bool;
    fn is_temporary_holiday(&self, date: &OffsetDateTime) -> bool;

    fn unpack(&self, date: &OffsetDateTime) -> (i32, Month, u8, Weekday, u16) {
        let year = date.year();
        let month = date.month();
        let day = date.day();
        let weekday = date.weekday();
        let day_of_year = date.ordinal();

        (year, month, day, weekday, day_of_year)
    }

    fn is_good_friday(&self, date: &OffsetDateTime, is_orthodox: bool) -> bool {
        let (year, _, _, _, dd) = self.unpack(date);

        if (year < FIRST_EASTER_MONDAY as i32) || (year as usize> LAST_EASTER_MONDAY) {
            warn!("Good Friday is not available for the year {}", year);
            return false;
        }

        if is_orthodox {
            let res = dd == EASTER_MONDAYS[0][year as usize - FIRST_EASTER_MONDAY] - 3;
            return res;
        } else {
            let res = dd == EASTER_MONDAYS[1][year as usize - FIRST_EASTER_MONDAY] - 3;
            return res;
        }
    }
}

pub struct NullCalendarType {}
impl Holidays for NullCalendarType {
    fn is_holiday(&self, _date: &OffsetDateTime) -> bool { false }
    fn is_temporary_holiday(&self, _date: &OffsetDateTime) -> bool { false }
}
pub trait Calendar {
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

    fn calendar_name(&self) -> &str;
    fn add_holidays(&mut self, date: &Date);
    fn remove_holidays(&mut self, date: &Date);

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

    fn year_fraction(&self, 
                    date_from: &OffsetDateTime, 
                    date_upto: &OffsetDateTime, 
                    day_count: &DayCountConvention) -> Real {
        // BoB
        let res = match day_count {
            DayCountConvention::Actual365Fixed => {
                let days = *date_upto - *date_from;
                days.whole_days() as Real / 365.0
                },
            DayCountConvention::Actual360 => {
                let days = *date_upto - *date_from;
                days.whole_days() as Real / 360.0
                },
            DayCountConvention::Actual364 => {
                let days = *date_upto - *date_from;
                days.whole_days() as Real / 364.0
                },
            DayCountConvention::Thirty360 => {
                let (year_from, month_from, day_from, _, _) = self.unpack_date(date_from);
                let (year_upto, month_upto, day_upto, _, _) = self.unpack_date(date_upto);
                let mut days = 0;
                days += 360 * (year_upto - year_from);
                days += 30 * (month_upto as i32 - month_from as i32);
                days += day_upto as i32 - day_from as i32;
                days as Real / 360.0
                },
            DayCountConvention::ActActIsda => {
                let days = (*date_upto - *date_from).whole_days() as Real;

                if self.is_leap_year(date_from.year()) {
                    days / 366.0
                } else {
                    days / 365.0
                }
                },
            };
        res        
    }

    //last_day_of month, not last business day of month
    fn last_day_of_month(&self, year: i32, month: Month) -> Date {
        match month {
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
        }
    }

    fn is_leap_year(&self, year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

}

pub struct NullCalendar {}
impl Calendar for NullCalendar {
    fn is_weekend(&self, date: &OffsetDateTime) -> bool {
        self._is_weekend(date)
    }
    fn is_holiday(&self, date: &OffsetDateTime) -> bool {
        self._is_holiday(date)
    }
    fn is_removed_holiday(&self, _date: &OffsetDateTime) -> bool { false }
    fn is_added_holiday(&self, _date: &OffsetDateTime) -> bool { false }
    fn is_base_holiday(&self, _date: &OffsetDateTime) -> bool { false }
    fn calendar_name(&self) -> &str { "NullCalendar" }
    fn add_holidays(&mut self, _date: &Date) {}
    fn remove_holidays(&mut self, _date: &Date) {}
    fn display_holidays(&self, 
                        date_from: &OffsetDateTime, 
                        date_upto: &OffsetDateTime,
                        include_weekend: bool) {
        self._display_holidays(date_from, date_upto, include_weekend);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::{date, datetime};

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
}