use crate::definitions::Time;
use crate::time::calendar::Calendar;
use crate::time::calendars::nullcalendar::NullCalendar;
use crate::time::calendars::southkorea::SouthKorea;
use crate::time::calendars::unitedstates::UnitedStates;
use crate::time::conventions::BusinessDayConvention;
use crate::time::conventions::DayCountConvention;
use anyhow::{anyhow, Result};
use enum_dispatch;
use time::{Date, Month, OffsetDateTime, Weekday};

#[enum_dispatch::enum_dispatch]
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

    fn display_holidays(
        &self,
        date_from: &OffsetDateTime,
        date_upto: &OffsetDateTime,
        include_weekend: bool,
    );

    fn _display_holidays(
        &self,
        date_from: &OffsetDateTime,
        date_upto: &OffsetDateTime,
        include_weekend: bool,
    ) {
        let mut date = *date_from;
        while date <= *date_upto {
            if self.is_holiday(&date) && (include_weekend || !self.is_weekend(&date)) {
                println!("{:?}", date);
            }
            date += time::Duration::days(1);
        }
    }

    fn is_removed_holiday(&self, date: &OffsetDateTime) -> bool;
    fn is_added_holiday(&self, date: &OffsetDateTime) -> bool;
    fn is_base_holiday(&self, date: &OffsetDateTime) -> bool;

    fn is_business_day(&self, date: &OffsetDateTime) -> bool {
        !self.is_holiday(date)
    }

    fn calendar_name(&self) -> &String;
    fn add_holidays(&mut self, date: &Date) -> Result<()>;
    fn remove_holidays(&mut self, date: &Date) -> Result<()>;

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
        false
    }

    fn adjust_following(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let mut res = *date;

        while self.is_holiday(&res) {
            res += time::Duration::days(1);
        }
        res
    }

    fn adjust_preceding(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let mut res = *date;

        while self.is_holiday(&res) {
            res -= time::Duration::days(1);
        }
        res
    }

    fn adjust_modified_following(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let month = date.month();

        let mut res = *date;

        while self.is_holiday(&res) {
            res += time::Duration::days(1);
        }

        if month != res.month() {
            res = *date;
            self.adjust_preceding(&res)
        } else {
            res
        }
    }

    fn adjust_modified_preceding(&self, date: &OffsetDateTime) -> OffsetDateTime {
        let month = date.month();

        let mut res = *date;

        while self.is_holiday(&res) {
            res -= time::Duration::days(1);
        }

        if month != res.month() {
            res = *date;
            self.adjust_following(&res)
        } else {
            res
        }
    }

    fn adjust(
        &self,
        date: &OffsetDateTime,
        convention: &BusinessDayConvention,
    ) -> Result<OffsetDateTime> {
        match convention {
            BusinessDayConvention::Unadjusted => Ok(*date),
            BusinessDayConvention::Following => Ok(self.adjust_following(date)),
            BusinessDayConvention::Preceding => Ok(self.adjust_preceding(date)),
            BusinessDayConvention::ModifiedPreceding => Ok(self.adjust_modified_preceding(date)),
            BusinessDayConvention::ModifiedFollowing => Ok(self.adjust_modified_following(date)),
            BusinessDayConvention::Dummy => {
                Err(anyhow!("Dummy business day convention is not supported"))
            }
        }
    }

    /// This year fraction is calculated mostly for coupon amount
    /// Thus, the functino considers only the days between date_from and date_upto
    /// For the exact time, use get_time_difference function (calculated by ActACtIsda Fasion)
    fn year_fraction(
        &self,
        start_date: &OffsetDateTime,
        end_date: &OffsetDateTime,
        day_count: &DayCountConvention,
    ) -> Result<Time> {
        // sanity check
        if start_date > end_date {
            return Err(anyhow!("{} > {}", start_date, end_date));
        }

        let res = match day_count {
            DayCountConvention::Actual365Fixed => {
                let days = *end_date - *start_date;
                days.whole_days() as Time / 365.0
            }
            DayCountConvention::Actual360 => {
                let days = *end_date - *start_date;
                days.whole_days() as Time / 360.0
            }
            DayCountConvention::Actual364 => {
                let days = *end_date - *start_date;
                days.whole_days() as Time / 364.0
            }
            DayCountConvention::Thirty360 => {
                let (year_from, month_from, day_from, _, _) = self.unpack_date(start_date);
                let (year_upto, month_upto, day_upto, _, _) = self.unpack_date(end_date);
                let mut days = 0;
                days += 360 * (year_upto - year_from);
                days += 30 * (month_upto as i32 - month_from as i32);
                days += day_upto as i32 - day_from as i32;
                days as Time / 360.0
            }
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
                    let days_in_year = if self.is_leap_year(year) {
                        366.0
                    } else {
                        365.0
                    };

                    if year == start_year {
                        let days = if end_year == start_year {
                            (*end_date - *start_date).whole_days() as Time
                        } else {
                            let last_day_of_year =
                                Date::from_calendar_date(year, Month::December, 31).unwrap();
                            (last_day_of_year - start_date.date()).whole_days() as Time
                        };
                        frac += days / days_in_year;
                    } else if year == end_year {
                        let last_day_of_previous_year =
                            Date::from_calendar_date(year - 1, Month::December, 31).unwrap();
                        let days =
                            (end_date.date() - last_day_of_previous_year).whole_days() as Time;
                        frac += days / days_in_year;
                    } else {
                        frac += 1.0; // Full year
                    };
                }
                frac
            }
            // 30/360 but with EOM
            DayCountConvention::StreetConvention => {
                let (year_from, month_from, day_from, _, _) = self.unpack_date(start_date);
                let (year_upto, month_upto, day_upto, _, _) = self.unpack_date(end_date);
                let mut days = 0;
                days += 360 * (year_upto - year_from);
                days += 30 * (month_upto as i32 - month_from as i32);
                days += day_upto as i32 - day_from as i32;
                if day_from == 31 {
                    if day_upto == 31 {
                        days -= 30;
                    } else {
                        days += 1;
                    }
                }
                days as Time / 360.0
            }
            DayCountConvention::Dummy => {
                return Err(anyhow!("Dummy day count convention is not supported"));
            }
        };
        Ok(res)
    }

    /// The last calendar day of a month, not the last business day
    fn last_day_of_month(&self, year: i32, month: Month) -> Date {
        match month {
            Month::January
            | Month::March
            | Month::May
            | Month::July
            | Month::August
            | Month::October
            | Month::December => Date::from_calendar_date(year, month, 31).unwrap(),

            Month::April | Month::June | Month::September | Month::November => {
                Date::from_calendar_date(year, month, 30).unwrap()
            }
            Month::February => {
                if self.is_leap_year(year) {
                    Date::from_calendar_date(year, month, 29).unwrap()
                } else {
                    Date::from_calendar_date(year, month, 28).unwrap()
                }
            }
        }
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
            let days_in_year: Time = if self.is_leap_year(year) {
                366.0
            } else {
                365.0
            };
            let start_date_offset = start_date.offset();
            if year == start_year {
                let days = if end_year == start_year {
                    (*end_date - *start_date).as_seconds_f64() as Time / (24.0 * 60.0 * 60.0)
                } else {
                    let last_day_of_year_start =
                        Date::from_calendar_date(year, Month::December, 31).unwrap();
                    let last_day_of_year = OffsetDateTime::new_in_offset(
                        last_day_of_year_start,
                        time::Time::MIDNIGHT,
                        start_date_offset,
                    );
                    (last_day_of_year - *start_date).as_seconds_f64() as Time / (24.0 * 60.0 * 60.0)
                };

                frac += days as Time / days_in_year;
            } else if year == end_year {
                let _last_day_of_previous_year =
                    Date::from_calendar_date(year - 1, Month::December, 31).unwrap();
                let last_day_of_previous_year = OffsetDateTime::new_in_offset(
                    _last_day_of_previous_year,
                    time::Time::MIDNIGHT,
                    start_date_offset,
                );
                let days = (*end_date - last_day_of_previous_year).as_seconds_f64() as Time
                    / (24.0 * 60.0 * 60.0);
                frac += days / days_in_year;
            } else {
                frac += 1.0; // Full year
            };
        }
        frac
    }
}
