use time::{OffsetDateTime, Weekday, Month};
use crate::time::constants::{EASTER_MONDAYS, FIRST_EASTER_MONDAY, LAST_EASTER_MONDAY};
use log::warn;

pub trait Holidays {
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
