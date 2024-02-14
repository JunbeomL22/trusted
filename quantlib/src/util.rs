use time::{Duration, Weekday, Month};
use regex::Regex;
use std::error::Error;

fn parse_duration(input: &str) -> Result<Duration, Box<dyn Error>> {
    let re = Regex::new(r"(?i)(\d+)(y|Y|M|m|D|d|H|h|min|s|sec)")?;
    let mut duration = Duration::zero();

    for cap in re.captures_iter(input) {
        let value = cap[1].parse::<i64>()?; // Use i64 to match the type expected by time::Duration
        match &cap[2].to_lowercase()[..] {
            "y" => duration += Duration::years(value),
            "m" => duration += Duration::months(value),
            "d" => duration += Duration::days(value),
            "h" => duration += Duration::hours(value),
            "min" => duration += Duration::minutes(value),
            "s" | "sec" => duration += Duration::seconds(value),
            // You might adjust the year/month approximation as needed
            _ => return Err("Unrecognized time unit".into()),
        }
    }

    Ok(duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        let input = "2M";
        let result = parse_duration(input);
        assert_eq!(result, Ok(Duration {months: 2, days: 0, hours: 0, minutes: 0, seconds: 0, milliseconds: 0, microseconds: 0, nanoseconds: 0}));

        let input = "1y2m";
        let result = parse_duration(input);
        assert_eq!(result, Ok(Duration {years: 1, months: 2, days: 0, hours: 0, minutes: 0, seconds: 0, milliseconds: 0, microseconds: 0, nanoseconds: 0}));
    }
}