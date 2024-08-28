use crate::time::calendar_trait::CalendarTrait;
use crate::utils::string_arithmetic::{
    from_month_to_i32,
    from_i32_to_month,
};
//
use serde::{Deserialize, Serialize};
use anyhow::Result;
use time::{Date, OffsetDateTime};

pub type Tenor = Period;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Period {
    years: i32,
    months: i32,
    days: i32,
}

impl Period {
    pub fn new(years: i32, months: i32, days: i32) -> Period {
        Period {
            years,
            months,
            days,
        }
    }

    /// "1Y" -> Period(1, 0, 0, 0)
    /// "1M" -> Period(0, 1, 0, 0)
    /// "1W" -> Period(0, 0, 7, 0)
    /// "1D" -> Period(0, 0, 1, 0)
    /// "1h" -> Period(0, 0, 0, 1)
    /// "1Y1M1W1D1h" -> Period(1, 1, 8, 1)
    pub fn new_from_string(tenor: &str) -> Result<Period> {
        let mut years = 0;
        let mut months = 0;
        let mut days = 0;
        //let mut hours = 0;

        let mut num = 0;
        for c in tenor.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                match c {
                    'Y' => years = num,
                    'M' => months = num,
                    'W' => days = num * 7,
                    'D' => days = num,
                    //'h' => hours = num,
                    _ => {
                        let err = || anyhow::anyhow!("Invalid tenor string: {}", tenor);
                        return Err(err());
                    }
                }
                num = 0;
            }
        }

        Ok(Period {
            years,
            months,
            days,
            //hours,
        })
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if self.years != 0 {
            result.push_str(&self.years.to_string());
            result.push('Y');
        }
        if self.months != 0 {
            result.push_str(&self.months.to_string());
            result.push('M');
        }
        if self.days != 0 {
            result.push_str(&self.days.to_string());
            result.push('D');
        }
        result
    }

    #[inline]
    #[must_use]
    pub fn years(&self) -> i32 {
        self.years
    }

    #[inline]
    #[must_use]
    pub fn months(&self) -> i32 {
        self.months
    }

    #[inline]
    #[must_use]
    pub fn days(&self) -> i32 {
        self.days
    }

    pub fn apply_year(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        if self.years == 0 {
            datetime.clone()
        } else {
            let mut new_datetime = datetime.clone();
            let new_year = new_datetime.year() + self.years;
            let new_month = new_datetime.month();
            let eom_new = crate::NULL_CALENDAR
                .last_day_of_month(new_year, new_month)
                .day();
            let new_day = match new_datetime.day() > eom_new {
                true => eom_new,
                false => new_datetime.day(),
            };
            new_datetime = OffsetDateTime::new_in_offset(
                Date::from_calendar_date(new_year, new_month, new_day)
                    .expect("Failed to create Date"),
                datetime.time(),
                datetime.offset(),
            );

            new_datetime
        } 
    }

    pub fn apply_month(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        if self.months == 0 {
            datetime.clone()
        } else {
            let mut new_datetime = datetime.clone();
            let new_year = new_datetime.year();
            let month_i32 = from_month_to_i32(new_datetime.month());
            let mut new_month_i32 = (month_i32 + self.months) % 12;
            new_month_i32 = if new_month_i32 < 0 { new_month_i32 + 12 } else { new_month_i32 };
            let new_month = from_i32_to_month(new_month_i32);
            let eom_new = crate::NULL_CALENDAR
                .last_day_of_month(new_year, new_month)
                .day();

            let new_day = match new_datetime.day() > eom_new {
                true => eom_new,
                false => new_datetime.day(),
            };
            new_datetime = OffsetDateTime::new_in_offset(
                Date::from_calendar_date(new_year, new_month, new_day)
                    .expect("Failed to create Date"),
                datetime.time(),
                datetime.offset(),
            );

            new_datetime
        }
    }

    pub fn apply_day(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        if self.days == 0 {
            datetime.clone()
        } else {
            let mut new_datetime = datetime.clone();
            let new_date = new_datetime.date() + time::Duration::days(self.days as i64);
            new_datetime = OffsetDateTime::new_in_offset(
                new_date,
                datetime.time(),
                datetime.offset(),
            );

            new_datetime
        }
    }

    pub fn apply(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        let mut new_datetime = self.apply_year(datetime);
        new_datetime = self.apply_month(&new_datetime);
        new_datetime = self.apply_day(&new_datetime);
        new_datetime
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct FinerPeriod {
    period: Period,
    hours: i32,
    minutes: i32,
    milli_seconds: i32,
    nano_seconds: i32,
}

impl FinerPeriod {
    pub fn new(period: Period, hours: i32, minutes: i32, milli_seconds: i32, nano_seconds: i32) -> FinerPeriod {
        FinerPeriod {
            period,
            hours,
            minutes,
            milli_seconds,
            nano_seconds,
        }
    }

    pub fn apply(&self, datetime: &OffsetDateTime) -> OffsetDateTime {
        let mut new_datetime = self.period.apply(datetime);
        new_datetime = new_datetime + time::Duration::hours(self.hours as i64);
        new_datetime = new_datetime + time::Duration::minutes(self.minutes as i64);
        new_datetime = new_datetime + time::Duration::milliseconds(self.milli_seconds as i64);
        new_datetime = new_datetime + time::Duration::nanoseconds(self.nano_seconds as i64);

        new_datetime
    }

    #[inline]
    #[must_use]
    pub fn period(&self) -> &Period {
        &self.period
    }

    #[inline]
    #[must_use]
    pub fn hours(&self) -> i32 {
        self.hours
    }

    #[inline]
    #[must_use]
    pub fn minutes(&self) -> i32 {
        self.minutes
    }

    #[inline]
    #[must_use]
    pub fn seconds(&self) -> i32 {
        self.milli_seconds / 1000
    }

    #[inline]
    #[must_use]
    pub fn milli_seconds(&self) -> i32 {
        self.milli_seconds
    }

    #[inline]
    #[must_use]
    pub fn micro_seconds(&self) -> i32 {
        self.nano_seconds / 1000
    }
    
    #[inline]
    #[must_use]
    pub fn nano_seconds(&self) -> i32 {
        self.nano_seconds
    }

    pub fn new_from_string(val: &str) -> Result<Self> {
        let mut period = Period::default();
        let mut hours = 0;
        let mut minutes = 0;
        let mut milli_seconds = 0;
        let mut nano_seconds = 0;

        let mut num = 0;
        let mut is_period = true;
        for c in val.chars() {
            if c.is_digit(10) {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            } else {
                match c {
                    'Y' => {
                        period.years = num;
                        is_period = true;
                    },
                    'M' => {
                        period.months = num;
                        is_period = true;
                    },
                    'W' => {
                        period.days = num * 7;
                        is_period = true;
                    },
                    'D' => {
                        period.days = num;
                        is_period = true;
                    },
                    'h' => {
                        hours = num;
                        is_period = false;
                    },
                    'm' => {
                        minutes = num;
                        is_period = false;
                    },
                    's' => {
                        milli_seconds += num * 1000;
                        is_period = false;
                    },
                    'l' => {
                        milli_seconds += num;
                        is_period = false;
                    },
                    'u' => {
                        nano_seconds += num * 1000;
                        is_period = false;
                    },
                    'n' => {
                        nano_seconds += num;
                        is_period = false;
                    },
                    _ => {
                        let err = || anyhow::anyhow!("Invalid tenor string: {}", val);
                        return Err(err());
                    }
                }
                num = 0;
            }
        }

        if is_period {
            Ok(FinerPeriod::new(period, hours, minutes, milli_seconds, nano_seconds))
        } else {
            let err = || anyhow::anyhow!("Invalid tenor string: {}", val);
            Err(err())
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    fn test_month_opration() {
        let datetime = datetime!(2023-12-31 00:00:00 +09:00);
        for i in 1..=36 {
            let period = Period::new(0, i, 0);
            let month = from_i32_to_month(i % 12);
            let new_datetime = period.apply(&datetime);

            assert_eq!(new_datetime.month(), month);
        }
    }

    #[test]
    fn test_year_opration() {
        let datetime = datetime!(2023-12-31 00:00:00 +09:00);
        for i in 1..=36 {
            let period = Period::new(i, 0, 0);
            let year = 2023 + i;
            let new_datetime = period.apply(&datetime);

            assert_eq!(new_datetime.year(), year);
        }
    }
}