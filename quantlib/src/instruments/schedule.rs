use crate::utils::myerror::MyError;
use anyhow::Result;
use time::{OffsetDateTime, Duration};
use serde::{Deserialize, Serialize};
use crate::time::calendar::Calendar;
use crate::time::conventions::{PaymentFrequency, BusinessDayConvention};
use crate::utils::string_arithmetic::add_period;
use std::ops::Index;
use crate::definitions::COUPON_PAYMENT_TIME;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BaseSchedule {   
    fixing_date: OffsetDateTime,
    calc_start_date: OffsetDateTime,
    calc_end_date: OffsetDateTime,
    payment_date: OffsetDateTime,
}

impl BaseSchedule {
    pub fn new(fixing_date: OffsetDateTime, calc_start_date: OffsetDateTime, calc_end_date: OffsetDateTime, payment_date: OffsetDateTime) -> Self {
        BaseSchedule { fixing_date, calc_start_date, calc_end_date, payment_date }
    }

    pub fn get_fixing_date(&self) -> &OffsetDateTime {
        &self.fixing_date
    }

    pub fn get_calc_start_date(&self) -> &OffsetDateTime {
        &self.calc_start_date
    }

    pub fn get_calc_end_date(&self) -> &OffsetDateTime {
        &self.calc_end_date
    }

    pub fn get_payment_date(&self) -> &OffsetDateTime {
        &self.payment_date
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Schedule {
    data: Vec<BaseSchedule>,
}

impl Index<usize> for Schedule {
    type Output = BaseSchedule;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl Default for Schedule {
    fn default() -> Self {
        Schedule { data: vec![] }
    }
}

impl Schedule {
    pub fn new(data: Vec<BaseSchedule>) -> Self {
        Schedule { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// make a schedule for a coupon for bonds, IRS, etc.
/// The first calc_start_date is the effective date
/// Then, the payment_dates are the calc_end_date + payment_gap adjusted by the BusinessDayConvention

pub fn build_schedule(
    effective_date: OffsetDateTime, 
    first_coupon_date: Option<OffsetDateTime>, 
    maturity: OffsetDateTime, 
    calendar: Calendar,
    conv: BusinessDayConvention,
    freq: PaymentFrequency, 
    fixing_days: i64, 
    payment_days: i64
) -> Result<Schedule, MyError> {
    if payment_days < 0 || fixing_days < 0 {
        // display all inputs. file and line are automatically filled by MyError
        let msg = String::from("payment_days and fixing_days should be non-negative\n");
        msg.push_str(&format!("effective_date: {:?}\n", effective_date));
        msg.push_str(&format!("first_coupon_date: {:?}\n", first_coupon_date));
        msg.push_str(&format!("maturity: {:?}\n", maturity));
        msg.push_str(&format!("calendar: {:?}\n", calendar.as_trait().calendar_name()));
        msg.push_str(&format!("conv: {:?}\n", conv));
        msg.push_str(&format!("freq: {:?}\n", freq));
        msg.push_str(&format!("fixing_days: {:?}\n", fixing_days));
        msg.push_str(&format!("payment_days: {:?}\n", payment_days));
        return Err(MyError::BaseError { 
            file: file!().to_string(),
            line: line!(),
            contents: msg,
        });
    }

    let mut data = Vec::new();
    let mut fixing_date = effective_date - Duration::days(fixing_days);
    let mut calc_start_date = effective_date;
    let mut raw_end = add_period(&effective_date, &freq.to_string());

    let mut calc_end_date = match first_coupon_date {
        Some(date) => date,
        None => if raw_end < maturity { raw_end } else { maturity },
    };

    let _payment_date = calc_end_date + Duration::days(payment_days);
    let payment_date = OffsetDateTime::new_in_offset(
        _payment_date.date(), 
        COUPON_PAYMENT_TIME,
        _payment_date.offset()
    );

    let base_schedule = BaseSchedule {
        fixing_date,
        calc_start_date,
        calc_end_date,
        payment_date,
    };

    data.push(base_schedule);

    let eps = Duration::hours(6);
    while calc_end_date < maturity - eps {
        calc_start_date = calc_end_date;
        raw_end = add_period(&calc_end_date, &freq.to_string());

        if raw_end < maturity - eps { 
            calc_end_date = calendar.as_trait().adjust(&raw_end, &conv);
        } else { 
            calc_end_date = maturity 
        };

        fixing_date = calc_start_date -  Duration::days(fixing_days);
        let payment_date = calc_end_date + Duration::days(payment_days);


        let base_schedule = BaseSchedule {
            fixing_date,
            calc_start_date,
            calc_end_date,
            payment_date,
        };

        data.push(base_schedule);
    }

    Ok(Schedule { data })
}

#[cfg(test)] 
mod tests {
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use super::*;
    use time::macros::datetime;

    // make a test for
    // calendar = SouthKorea::new(SouthKoreaType::Settlement)
    // effective_date = date!(2023-08-02 16:00:00 +09:00)
    // frequency = PaymentFrequency::Quarterly
    // fix days = 1
    // pay days = 0
    // maturity = date!(2024-02-05 16:00:00 +09:00)
    // conv = ModifiedFollowing
    #[test]
    fn test_build_schedule() {
        let effective_date = datetime!(2023-08-03 16:00:00 +09:00);
        let maturity = datetime!(2024-02-05 16:00:00 +09:00);
        let calendar: &dyn Calendar = &SouthKorea::new(SouthKoreaType::Settlement);
        let schedule = build_schedule(
            effective_date, 
            None, 
            maturity, 
            calendar,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            1, 
            0);

        let expected_schedule = Schedule {
            data: vec![
                BaseSchedule {
                    fixing_date: datetime!(2023-08-02 16:00:00.0 +09:00:00),
                    calc_start_date: datetime!(2023-08-03 16:00:00.0 +09:00:00),
                    calc_end_date: datetime!(2023-11-03 16:00:00.0 +09:00:00),
                    payment_date: datetime!(2023-11-03 16:00:00.0 +09:00:00),
                },
                BaseSchedule {
                    fixing_date: datetime!(2023-11-02 16:00:00.0 +09:00:00),
                    calc_start_date: datetime!(2023-11-03 16:00:00.0 +09:00:00),
                    calc_end_date: datetime!(2024-02-05 16:00:00.0 +09:00:00),
                    payment_date: datetime!(2024-02-05 16:00:00.0 +09:00:00),
                },
            ],
        };

        for i in 0..schedule.len() {
            assert_eq!(
                schedule[i].get_fixing_date(), 
                expected_schedule[i].get_fixing_date(),
                "{}-th fixing date: {:?}, expected: {:?}",
                i,
                schedule[i].get_fixing_date(),
                expected_schedule[i].get_fixing_date()
            );

            assert_eq!(
                schedule[i].get_calc_start_date(), 
                expected_schedule[i].get_calc_start_date(),
                "{}-th calc_start_date: {:?}, expected: {:?}",
                i,
                schedule[i].get_calc_start_date(),
                expected_schedule[i].get_calc_start_date()
            );

            assert_eq!(
                schedule[i].get_calc_end_date(), 
                expected_schedule[i].get_calc_end_date(),
                "{}-th calc_end_date: {:?}, expected: {:?}",
                i,
                schedule[i].get_calc_end_date(),
                expected_schedule[i].get_calc_end_date()
            );

            assert_eq!(
                schedule[i].get_payment_date(), 
                expected_schedule[i].get_payment_date(),
                "{}-th payment_date: {:?}, expected: {:?}",
                i,
                schedule[i].get_payment_date(),
                expected_schedule[i].get_payment_date()
            );
        }
        
    }
    
    
}