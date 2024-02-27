use time::OffsetDateTime;
use serde::{Deserialize, Serialize};
use crate::time::calendar::Calendar;
use crate::conventions::{PaymentFrequency, BusinessDayConvention};
use crate::utils::string_arithmetic::add_period;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
    calendar: &Calendar, 
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
    while calc_start_date < end_date {
        let calc_end_date = add_period(calc_start_date, freq.to_string());
        payment_date = calc_end_date;
        fixing_date = calc_start_date;
        data.push(TimeData::new(fixing_date, calc_start_date, calc_end_date, payment_date));
        calc_start_date = calendar.adjust(calc_end_date, conv);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::time::calendar::Calendar;
    use crate::conventions::{PaymentFrequency, BusinessDayConvention};
    use time::OffsetDateTime;
    #[test]
    fn test_make_schedule() {
        let calendar = Calendar::new();
        let start_date = OffsetDateTime::from_iso_date_string("2022-01-01").unwrap();
        let effective_date = OffsetDateTime::from_iso_date_string("2022-01-01").unwrap();
        let payment_gap = 3;
        let end_date = OffsetDateTime::from_iso_date_string("2022-12-31").unwrap();
        let freq = PaymentFrequency::SemiAnnually;
        let conv = BusinessDayConvention::Following;
        let schedule = make_schedule(&calendar, start_date, effective_date, payment_gap, end_date, freq, conv);
        assert_eq!(schedule.data.len(), 2);
    }
}
