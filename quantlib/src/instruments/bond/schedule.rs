use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use crate::time::calendar::Calendar;
use crate::time::conventions::{PaymentFrequency, BusinessDayConvention};
use crate::utils::string_arithmetic::add_period;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeData {
    fixing_date: OffsetDateTime,
    calc_start_date: OffsetDateTime,
    calc_end_date: OffsetDateTime,
    payment_date: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    data: Vec<TimeData>,
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule { data: vec![] }
    }
}

impl TimeData {
    pub fn new(fixing_date: OffsetDateTime, calc_start_date: OffsetDateTime, calc_end_date: OffsetDateTime, payment_date: OffsetDateTime) -> Self {
        TimeData { fixing_date, calc_start_date, calc_end_date, payment_date }
    }
}

impl Schedule {
    pub fn new(data: Vec<TimeData>) -> Self {
        Schedule { data }
    }
}

/// make a schedule for a coupon for bonds, IRS, etc.
/// The first calc_start_date is the effective date
/// Then, the payment_dates are the calc_end_date + payment_gap adjusted by the BusinessDayConvention
pub fn make_schedule(
    calendar: Box<dyn Calendar>, 
    start_date: OffsetDateTime, 
    effective_date: OffsetDateTime,
    payment_gap: i16,
    end_date: OffsetDateTime, 
    freq: PaymentFrequency,
    conv: BusinessDayConvention) -> Schedule {
    let mut data = vec![];
    let mut calc_start_date = effective_date;
    let mut payment_date = effective_date;
    let mut fixing_date = start_date;
    
    while payment_date < end_date {
        let calc_end_date = add_period(&calc_start_date, freq.to_string().as_str());
        payment_date = add_period(&payment_date, freq.to_string().as_str());
        let payment_date = calendar.adjust(&payment_date, &conv);
        data.push(TimeData::new(fixing_date, calc_start_date, calc_end_date, payment_date));
        calc_start_date = calc_end_date;
    }
    Schedule::new(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::time::conventions::{PaymentFrequency, BusinessDayConvention};
    use time::macros::datetime;
    
    #[test]
    fn test_make_schedule() {
        let calendar = Box::new(SouthKorea::new(SouthKoreaType::Settlement));
        let start_date = datetime!(2022-01-01 00:00:00 +09:00);
        let effective_date = datetime!(2022-01-01 00:00:00 +09:00);
        let payment_gap = 2;
        let end_date = datetime!(2023-01-01 00:00:00 +09:00);
        let freq = PaymentFrequency::SemiAnnually;
        let conv = BusinessDayConvention::Following;
        let schedule = make_schedule(calendar, start_date, effective_date, payment_gap, end_date, freq, conv);
        assert_eq!(schedule.data.len(), 2);
    }
}
