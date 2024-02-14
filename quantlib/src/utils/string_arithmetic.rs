use time::{OffsetDateTime, Duration, Month};
use regex;


fn from_i32_to_month(item: i32) -> Month {
    match item {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("Invalid month number"),
    }
}

fn from_month_to_i32(month: Month) -> i32 {
    match month {
        Month::January => 1,
        Month::February => 2,
        Month::March => 3,
        Month::April => 4,
        Month::May => 5,
        Month::June => 6,
        Month::July => 7,
        Month::August => 8,
        Month::September => 9,
        Month::October => 10,
        Month::November => 11,
        Month::December => 12,
        _ => panic!("Invalid month"),
    }
}

/// # Examples
/// ```
/// use time::macros::datetime;
/// let x = datetime!(2021-01-01 00:00:00 UTC);
/// let y = add_period(x, "1y1m1D1h1min1sec");
/// println!("{}", y); // 2022-02-02 01:01:01 UTC
/// "Y" is year, "M" is month, "D" is day, "h" is hour, "min" is minute, "sec" is second. Y, M, D, h are case insensitive
/// This uses regularexpression to parse the string and add the duration to the datetime
/// ```
pub fn add_period(datetime: OffsetDateTime, duration: &str) -> OffsetDateTime {
    let re = regex::Regex::new(r"(\d+)(Y|y|M|m|D|d|H|h|min|sec)+").unwrap();
    let mut new_datetime = datetime;
    for cap in re.captures_iter(duration) {
        let value = cap[1].parse::<i64>().unwrap();
        let unit = &cap[2];
        match unit.to_lowercase().as_str() {
            "y" => {
                let new_year = new_datetime.year() + value as i32;
                new_datetime = new_datetime.replace_year(new_year).unwrap();
            },
            "m" => {
                assert!(1 <= value && value <= 12, "Month value must be less than 12");
                let month = from_month_to_i32(new_datetime.month());
                let year = new_datetime.year();
                let new_month = from_i32_to_month((month + value as i32) % 12);
                let new_year = year + (month + value as i32) / 12;
                new_datetime = new_datetime.replace_month(new_month).unwrap().replace_year(new_year).unwrap();
            },
            "d" => new_datetime = new_datetime + Duration::days(value),
            "h" => new_datetime = new_datetime + Duration::hours(value),
            "min" => new_datetime = new_datetime + Duration::minutes(value),
            "sec" => new_datetime = new_datetime + Duration::seconds(value),
            _ => panic!("Invalid unit"),
        }
    }
    new_datetime
}

/// # Examples
/// ```
/// use time::macros::datetime;
/// let x = datetime!(2021-01-01 00:00:00 UTC);
/// let y = sub_period(x, "1y1m1D1h1min1sec");
/// println!("{}", y); // 2019-11-29 22:58:59 UTC
/// "Y" is year, "M" is month, "D" is day, "h" is hour, "min" is minute, "sec" is second. Y, M, D, h are case insensitive
/// This uses regularexpression to parse the string and add the duration to the datetime
/// ```
pub fn sub_period(datetime: OffsetDateTime, duration: &str) -> OffsetDateTime {
    let re = regex::Regex::new(r"(\d+)(Y|y|M|m|D|d|H|h|min|sec)+)").unwrap();
    let mut new_datetime = datetime;
    for cap in re.captures_iter(duration) {
        let value = cap[1].parse::<i64>().unwrap();
        let unit = &cap[2];
        match unit.to_lowercase().as_str() {
            "y" => {
                let new_year = new_datetime.year() - value as i32;
                new_datetime = new_datetime.replace_year(new_year).unwrap();
            },
            "m" => {
                assert!(1 <= value && value <= 12, "Month value must be less than 12");
                let month = from_month_to_i32(new_datetime.month());
                let year = new_datetime.year();
                let new_month = from_i32_to_month((month - value as i32 + 12) % 12);
                let new_year = year - 1 + (month - value as i32 + 12) / 12;
                new_datetime = new_datetime.replace_month(new_month).unwrap().replace_year(new_year).unwrap();
            },
            "d" => new_datetime = new_datetime - Duration::days(value),
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
    
    #[test]
    fn test_add_period() {
        let x = datetime!(2021-01-01 00:00:00 UTC);
        let y = add_period(x, "1y1m1D1h1min1sec");
        assert_eq!(y, datetime!(2022-02-02 01:01:01 UTC));
    }

    #[test]
    fn test_sub_period() {
        let x = datetime!(2021-01-01 00:00:00 UTC);
        let y = sub_period(x, "1y1m1D1h1min1sec");
        assert_eq!(y, datetime!(2019-11-29 22:58:59 UTC));
    }
}