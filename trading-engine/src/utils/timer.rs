use quanta::Clock;
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use std::cell::{Cell, RefCell};

const UNIX_NANO_ANCHOR_BUFFER: u64 = 10; //10ns

thread_local! {
    static CLOCK: RefCell<Option<Clock>> = const {RefCell::new(None)};
    static SYSTEMTIME_ANCHOR: Cell<Option<u64>> = const {Cell::new(None)};
    static CLOCK_ANCHOR: Cell<Option<u64>> = const {Cell::new(None)};
}

pub fn get_thread_local_unix_nano() -> u64 {
    SYSTEMTIME_ANCHOR.with(|systemtime_anchor| {
        CLOCK_ANCHOR.with(|clock_anchor| {
            CLOCK.with(|clock| {
                if systemtime_anchor.get().is_none() {
                    let clock_content = Clock::new();
                    systemtime_anchor.set(Some(SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos() as u64 + UNIX_NANO_ANCHOR_BUFFER));
                    clock_anchor.set(Some(clock_content.raw()));
                    *clock.borrow_mut() = Some(clock_content);
                }

                let clock_borrow = clock.borrow();
                let clock_content = clock_borrow.as_ref().unwrap();
                clock_content.delta_as_nanos(clock_anchor.get().unwrap(), clock_content.raw()) + systemtime_anchor.get().unwrap()
            })
        })
    })
}

pub static UNIVERSIAL_CLOCK: Lazy<Clock> = Lazy::new(Clock::new);

#[inline]
pub fn get_unix_nano() -> u64 {
    static UNIVERSIAL_SYSTEMTIME_ANCHOR: Lazy<u64> = Lazy::new(|| {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64 + UNIX_NANO_ANCHOR_BUFFER
    });
    static UNIVERSIAL_CLOCK_ANCHOR: Lazy<u64> = Lazy::new(|| UNIVERSIAL_CLOCK.raw());

    UNIVERSIAL_CLOCK.delta_as_nanos(*UNIVERSIAL_CLOCK_ANCHOR, UNIVERSIAL_CLOCK.raw()) + *UNIVERSIAL_SYSTEMTIME_ANCHOR
}

pub fn convert_unix_nano_to_datetime_format(unix_nano: u64, utc_offset_hour: i32) -> String {
    const NANOS_IN_SEC: u64 = 1_000_000_000;
    const NANOS_IN_MIN: u64 = 60 * NANOS_IN_SEC;
    const NANOS_IN_HOUR: u64 = 60 * NANOS_IN_MIN;
    const NANOS_IN_DAY: u64 = 24 * NANOS_IN_HOUR;

    let days_since_epoch = unix_nano / NANOS_IN_DAY;
    let remaining_nanos = unix_nano % NANOS_IN_DAY;

    let hours = remaining_nanos / NANOS_IN_HOUR;
    let remaining_nanos = remaining_nanos % NANOS_IN_HOUR;

    let minutes = remaining_nanos / NANOS_IN_MIN;
    let remaining_nanos = remaining_nanos % NANOS_IN_MIN;

    let seconds = remaining_nanos / NANOS_IN_SEC;
    let remaining_nanos = remaining_nanos % NANOS_IN_SEC;

    let millis = remaining_nanos / 1_000_000;
    let remaining_nanos = remaining_nanos % 1_000_000;

    let micros = remaining_nanos / 1_000;
    let nanos = remaining_nanos % 1_000;

    // Adjust for UTC offset
    let mut total_hours = hours as i32 + utc_offset_hour;
    let mut total_days = days_since_epoch as i32;
    
    if total_hours >= 24 {
        total_hours -= 24;
        total_days += 1;
    } else if total_hours < 0 {
        total_hours += 24;
        total_days -= 1;
    }

    let (year, month, day) = days_to_date(total_days as u32);

    format!("{:04}{:02}{:02} {:02}:{:02}:{:02} {:03}:{:03}:{:03}", 
            year, month, day, total_hours, minutes, seconds, millis, micros, nanos)
}

fn days_to_date(mut days: u32) -> (i32, u32, u32) {
    let mut year = 1970;

    // Find the year
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    // Find the month and day
    let mut month = 1;
    while days > 0 {
        let days_in_month = days_in_month(year, month);
        if days < days_in_month {
            break;
        }
        days -= days_in_month;
        month += 1;
    }

    (year, month, days + 1)
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unix_nano() {
        let unix_nano = get_unix_nano();
        assert!(unix_nano > 0);
    }
}