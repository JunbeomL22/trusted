use anyhow::{Result, anyhow};
use time::{OffsetDateTime, Duration};
use serde::{Serialize, Deserialize};
use crate::time::jointcalendar::JointCalendar;
use crate::time::conventions::{PaymentFrequency, BusinessDayConvention};
use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::utils::string_arithmetic::add_period;
use std::ops::Index;
use crate::definitions::COUPON_PAYMENT_TIME;
use crate::definitions::Real;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct BaseSchedule {   
    fixing_date: OffsetDateTime,
    calc_start_date: OffsetDateTime,
    calc_end_date: OffsetDateTime,
    payment_date: OffsetDateTime,
    amount: Option<Real>, // if None, pricer calculate the coupon amount
}

impl BaseSchedule {
    pub fn new(
        fixing_date: OffsetDateTime, 
        calc_start_date: OffsetDateTime, 
        calc_end_date: OffsetDateTime, 
        payment_date: OffsetDateTime,
        amount: Option<Real>
    ) -> Self {
        BaseSchedule { fixing_date, calc_start_date, calc_end_date, payment_date, amount }
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

    pub fn get_amount(&self) -> Option<Real> {
        self.amount
    }

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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

impl<'a> IntoIterator for &'a Schedule {
    type Item = &'a BaseSchedule;
    type IntoIter = std::slice::Iter<'a, BaseSchedule>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl Schedule {
    pub fn new(data: Vec<BaseSchedule>) -> Self {
        Schedule { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<BaseSchedule> {
        self.data.iter()
    }
}

/// make a schedule for a coupon for bonds, IRS, etc.
/// The first calc_start_date is the effective date
/// Then, the payment_dates are the calc_end_date + payment_gap adjusted by the BusinessDayConvention

pub fn build_schedule(
    effective_date: &OffsetDateTime, 
    first_coupon_date: Option<&OffsetDateTime>, 
    maturity: &OffsetDateTime, 
    calendar: &JointCalendar,
    conv: &BusinessDayConvention,
    freq: &PaymentFrequency, 
    fixing_days: i64, 
    payment_days: i64
) -> Result<Schedule> {
    if payment_days < 0 || fixing_days < 0 {
        // display all inputs. file and line are automatically filled by MyError
        let mut msg = String::from("payment_days and fixing_days should be non-negative\n");
        msg.push_str(&format!("effective_date: {:?}\n", effective_date));
        msg.push_str(&format!("first_coupon_date: {:?}\n", first_coupon_date));
        msg.push_str(&format!("maturity: {:?}\n", maturity));
        msg.push_str(&format!("calendar: {:?}\n", calendar.calendar_name()));
        msg.push_str(&format!("conv: {:?}\n", conv));
        msg.push_str(&format!("freq: {:?}\n", freq));
        msg.push_str(&format!("fixing_days: {:?}\n", fixing_days));
        msg.push_str(&format!("payment_days: {:?}\n", payment_days));
        return Err(anyhow!(msg));
    }

    let mut data = Vec::new();
    let mut fixing_date = effective_date.clone() - Duration::days(fixing_days);
    let mut calc_start_date = effective_date.clone();
    let mut raw_end = add_period(&effective_date, &freq.to_string());
    let cloned_maturity = maturity.clone();
    let mut calc_end_date = match first_coupon_date {
        Some(date) => date.clone(),
        None => if raw_end < cloned_maturity { raw_end } else { cloned_maturity },
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
        amount: None,
    };

    data.push(base_schedule);

    let eps = Duration::hours(6);
    while calc_end_date < cloned_maturity - eps {
        calc_start_date = calc_end_date;
        raw_end = add_period(&calc_end_date, &freq.to_string());

        if raw_end < *maturity - eps { 
            calc_end_date = calendar.adjust(&raw_end, &conv);
        } else { 
            calc_end_date = cloned_maturity 
        };

        fixing_date = calc_start_date -  Duration::days(fixing_days);
        let payment_date = calc_end_date + Duration::days(payment_days);


        let base_schedule = BaseSchedule {
            fixing_date,
            calc_start_date,
            calc_end_date,
            payment_date,
            amount: None,
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
    use crate::time::calendar::Calendar;
    use crate::time::calendar::SouthKoreaWrapper;
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
        let cal = SouthKorea::new(SouthKoreaType::Settlement);
        let calendar = Calendar::SouthKorea(SouthKoreaWrapper{c: cal});
        let joint_calendar = JointCalendar::new(vec![calendar]);
        let schedule = build_schedule(
            &effective_date, 
            None, 
            &maturity, 
            &joint_calendar,
            &BusinessDayConvention::ModifiedFollowing,
            &PaymentFrequency::Quarterly,
            1, 
            0
        ).expect("Failed to build schedule");

        let expected_schedule = Schedule {
            data: vec![
                BaseSchedule {
                    fixing_date: datetime!(2023-08-02 16:00:00.0 +09:00:00),
                    calc_start_date: datetime!(2023-08-03 16:00:00.0 +09:00:00),
                    calc_end_date: datetime!(2023-11-03 16:00:00.0 +09:00:00),
                    payment_date: datetime!(2023-11-03 16:00:00.0 +09:00:00),
                    amount: None,
                },
                BaseSchedule {
                    fixing_date: datetime!(2023-11-02 16:00:00.0 +09:00:00),
                    calc_start_date: datetime!(2023-11-03 16:00:00.0 +09:00:00),
                    calc_end_date: datetime!(2024-02-05 16:00:00.0 +09:00:00),
                    payment_date: datetime!(2024-02-05 16:00:00.0 +09:00:00),
                    amount: None,
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