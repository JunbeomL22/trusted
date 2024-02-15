use time::{OffsetDateTime, Duration, Month};
use regex;


fn from_i32_to_month(item: i32) -> Month {
    match item {
        0 => Month::January,
        1 => Month::February,
        2 => Month::March,
        3 => Month::April,
        4 => Month::May,
        5 => Month::June,
        6 => Month::July,
        7 => Month::August,
        8 => Month::September,
        9 => Month::October,
        10 => Month::November,
        11 => Month::December,
        _ => panic!("Invalid month number: {}", item),
    }
}

fn from_month_to_i32(month: Month) -> i32 {
    match month {
        Month::January => 0,
        Month::February => 1,
        Month::March => 2,
        Month::April => 3,
        Month::May => 4,
        Month::June => 5,
        Month::July => 6,
        Month::August => 7,
        Month::September => 8,
        Month::October => 9,
        Month::November => 10,
        Month::December => 11,
    }
}

/// This uses regularexpression to parse the string and add the duration to the datetime
/// "Y" is year, "M" is month, "W" is week, "D" is day, "h" is hour, "min" is minute, "sec" is second. They are case sensitive
/// # Examples
/// ```
/// use time::macros::datetime;
/// use quantlib::utils::string_arithmetic::add_period;
/// 
/// let x = datetime!(2021-01-01 00:00:00 UTC);
/// let y = add_period(x, "1y1m1D1h1min1sec");
/// println!("{}", y); // 2022-02-02 01:01:01 UTC
/// ```
/// 
pub fn add_period(datetime: &OffsetDateTime, duration: &str) -> OffsetDateTime {
    let re = regex::Regex::new(r"(\d+)(Y|M|W|D|h|min|sec)+").unwrap();
    if !re.is_match(duration) {
        panic!("panic at add_period(datetime: {}, duration: {})", datetime, duration);
    }
    let mut new_datetime = *datetime;
    // panic where the duration is invalid
    for cap in re.captures_iter(duration) {
        let value = cap[1].parse::<i64>().unwrap();
        let unit = &cap[2];
        match unit {
            "Y" => {
                let new_year = new_datetime.year() + value as i32;
                new_datetime = new_datetime.replace_year(new_year).unwrap();
            },
            "M" => {
                assert!(
                    1 <= value && value <= 12, 
                    "(add_period) Month value must be less than 12. \ndatetime: {}, duration: {}, value: {}",
                    datetime, duration, value
                );
                let month_i32 = from_month_to_i32(new_datetime.month());
                let year = new_datetime.year();
                let new_month = from_i32_to_month((month_i32 + value as i32) % 12);
                let new_year = year + (month_i32 + value as i32) / 12;
                new_datetime = new_datetime.replace_month(new_month).unwrap().replace_year(new_year).unwrap();
            },
            "W" => new_datetime = new_datetime + Duration::weeks(value),
            "D" => new_datetime = new_datetime + Duration::days(value),
            "h" => new_datetime = new_datetime + Duration::hours(value),
            "min" => new_datetime = new_datetime + Duration::minutes(value),
            "sec" => new_datetime = new_datetime + Duration::seconds(value),
            _ => panic!("Invalid unit"),
        }
    }
    new_datetime
}

/// "Y" is year, "M" is month, "D" is day, "W" is week, "h" is hour, "min" is minute, "sec" is second. Y, M, D, h are case sensitive
/// This uses regularexpression to parse the string and add the duration to the datetime
/// # Examples
/// ```
/// use time::macros::datetime;
/// use quantlib::utils::string_arithmetic::sub_period;
/// 
/// let x = datetime!(2021-01-01 00:00:00 UTC);
/// let y = sub_period(x, "1y1m1D1h1min1sec");
/// println!("{}", y); // 2019-11-29 22:58:59 UTC
/// ```
/// 
pub fn sub_period(datetime: &OffsetDateTime, duration: &str) -> OffsetDateTime {
    let re = regex::Regex::new(r"(\d+)(Y|M|W|D|h|min|sec)+").unwrap();
    if !re.is_match(duration) {
        panic!("Invalid duration: {}", duration);
    }
    let mut new_datetime = *datetime;
    for cap in re.captures_iter(duration) {
        let value = cap[1].parse::<i64>().unwrap();
        let unit = &cap[2];
        match unit {
            "Y" => {
                let new_year = new_datetime.year() - value as i32;
                new_datetime = new_datetime.replace_year(new_year).unwrap();
            },
            "M" => {
                assert!(
                    1 <= value && value <= 12, 
                    "(sub_period) Month value must be less than 12. \ndatetime: {}, duration: {}, value: {}",
                    datetime, duration, value
                );
                let month_i32 = from_month_to_i32(new_datetime.month());
                let year = new_datetime.year();
                
                let new_month = from_i32_to_month((month_i32 - value as i32 + 12) % 12);
                let new_year = year - 1 + (month_i32 - value as i32 + 12) / 12;
                new_datetime = new_datetime.replace_month(new_month).unwrap().replace_year(new_year).unwrap();
            },
            "W" => new_datetime = new_datetime - Duration::weeks(value),
            "D" => new_datetime = new_datetime - Duration::days(value),
            "h" => new_datetime = new_datetime - Duration::hours(value),
            "min" => new_datetime = new_datetime - Duration::minutes(value),
            "sec" => new_datetime = new_datetime - Duration::seconds(value),
            _ => panic!("Invalid unit"),
        }
    }
    new_datetime
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    //use rstest::rstest;
    //use time::Month;

    #[test]
    fn test_month_operation() {
        let mut month = Month::January;
        let month_i32 = from_month_to_i32(month);
        assert_eq!(from_i32_to_month((month_i32 + 0) % 12), Month::January);
        assert_eq!(from_i32_to_month((month_i32 + 1) % 12), Month::February);
        assert_eq!(from_i32_to_month((month_i32 + 2) % 12), Month::March);
        assert_eq!(from_i32_to_month((month_i32 + 3) % 12), Month::April);
        assert_eq!(from_i32_to_month((month_i32 + 4) % 12), Month::May);
        assert_eq!(from_i32_to_month((month_i32 + 5) % 12), Month::June);
        assert_eq!(from_i32_to_month((month_i32 + 6) % 12), Month::July);
        assert_eq!(from_i32_to_month((month_i32 + 7) % 12), Month::August);
        assert_eq!(from_i32_to_month((month_i32 + 8) % 12), Month::September);
        assert_eq!(from_i32_to_month((month_i32 + 9) % 12), Month::October);
        assert_eq!(from_i32_to_month((month_i32 + 10) % 12), Month::November);
        assert_eq!(from_i32_to_month((month_i32 + 11) % 12), Month::December);
        assert_eq!(from_i32_to_month((month_i32 + 12) % 12), Month::January);
        assert_eq!(from_i32_to_month((month_i32 + 13) % 12), Month::February);
        assert_eq!(from_i32_to_month((month_i32 + 14) % 12), Month::March);
        assert_eq!(from_i32_to_month((month_i32 + 15) % 12), Month::April);
        assert_eq!(from_i32_to_month((month_i32 + 16) % 12), Month::May);
        assert_eq!(from_i32_to_month((month_i32 + 17) % 12), Month::June);
        assert_eq!(from_i32_to_month((month_i32 + 18) % 12), Month::July);
        assert_eq!(from_i32_to_month((month_i32 + 19) % 12), Month::August);
        assert_eq!(from_i32_to_month((month_i32 + 20) % 12), Month::September);
        assert_eq!(from_i32_to_month((month_i32 + 21) % 12), Month::October);
        assert_eq!(from_i32_to_month((month_i32 + 22) % 12), Month::November);
        assert_eq!(from_i32_to_month((month_i32 + 23) % 12), Month::December);
        assert_eq!(from_i32_to_month((month_i32 + 24) % 12), Month::January);

        month = Month::June;
        let month_i32 = from_month_to_i32(month);
        assert_eq!(from_i32_to_month((month_i32 + 0) % 12), Month::June);
        assert_eq!(from_i32_to_month((month_i32 + 1) % 12), Month::July);
        assert_eq!(from_i32_to_month((month_i32 + 2) % 12), Month::August);
        assert_eq!(from_i32_to_month((month_i32 + 3) % 12), Month::September);
        assert_eq!(from_i32_to_month((month_i32 + 4) % 12), Month::October);
        assert_eq!(from_i32_to_month((month_i32 + 5) % 12), Month::November);
        assert_eq!(from_i32_to_month((month_i32 + 6) % 12), Month::December);
        assert_eq!(from_i32_to_month((month_i32 + 7) % 12), Month::January);
        assert_eq!(from_i32_to_month((month_i32 + 8) % 12), Month::February);
        
        month = Month::November;
        let month_i32 = from_month_to_i32(month);
        assert_eq!(from_i32_to_month((month_i32 + 0) % 12), Month::November);
        assert_eq!(from_i32_to_month((month_i32 + 1) % 12), Month::December);
        assert_eq!(from_i32_to_month((month_i32 + 2) % 12), Month::January);
        assert_eq!(from_i32_to_month((month_i32 + 3) % 12), Month::February);

        month = Month::December;
        let month_i32 = from_month_to_i32(month);
        assert_eq!(from_i32_to_month((month_i32 + 0) % 12), Month::December);
        assert_eq!(from_i32_to_month((month_i32 + 1) % 12), Month::January);
        assert_eq!(from_i32_to_month((month_i32 + 2) % 12), Month::February);
        assert_eq!(from_i32_to_month((month_i32 + 3) % 12), Month::March);
        
    }

    #[test]
    fn test_add_period() {
        let x = datetime!(2021-01-01 00:00:00 UTC);
        let y = add_period(&x, "1Y1M1D1h1min1sec");
        assert_eq!(y, datetime!(2022-02-02 01:01:01 UTC));

        let mut y = add_period(&x, "1M");
        assert_eq!(y, datetime!(2021-02-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-03-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-04-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-05-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-06-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-07-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-08-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-09-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-10-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-11-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2021-12-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2022-01-01 00:00:00 UTC));
        y = add_period(&y, "1M");
        assert_eq!(y, datetime!(2022-02-01 00:00:00 UTC));
    }

    #[test]
    fn test_sub_period() {
        let x = datetime!(2021-01-01 00:00:00 UTC);
        let y = sub_period(&x, "1Y1M1D1h1min1sec");
        assert_eq!(y, datetime!(2019-11-29 22:58:59 UTC));

        let y = sub_period(&x, "1M");
        assert_eq!(y, datetime!(2020-12-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-11-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-10-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-09-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-08-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-07-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-06-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-05-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-04-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-03-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-02-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2020-01-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2019-12-01 00:00:00 UTC));
        let y = sub_period(&y, "1M");
        assert_eq!(y, datetime!(2019-11-01 00:00:00 UTC));
    }

    #[test]
    fn test_add_week_in_leap_year() {
        let x = datetime!(2024-02-28 00:00:00 UTC);
        let y = add_period(&x, "1W");
        assert_eq!(y, datetime!(2024-03-06 00:00:00 UTC));

        let x = datetime!(2023-02-28 00:00:00 UTC);
        let y = add_period(&x, "1W");
        assert_eq!(y, datetime!(2023-03-07 00:00:00 UTC));
    }
}