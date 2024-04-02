use crate::time::jointcalendar::JointCalendar;
use crate::time::conventions::{PaymentFrequency, BusinessDayConvention};
use crate::time::calendar_trait::CalendarTrait;
use crate::utils::string_arithmetic::{add_period, sub_period};
use crate::definitions::COUPON_PAYMENT_TIME;
use crate::definitions::Real;
//
use anyhow::{Result, anyhow};
use time::{OffsetDateTime, Duration};
use serde::{Serialize, Deserialize};
use std::{
    ops::Index,
    collections::VecDeque,
};

/// if the amount is None, the pricer calculate the coupon amount. 
/// Otherwise, the amount is used as the coupon amount. 
/// This is useful if the user wants to calculate the coupon amount 
/// in the IO section, e.g., serialization, deserialization, etc.
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
    forward_generation: bool, // true => generate from effective_date to maturity, false => generate from maturity to effective_date
    effective_date: &OffsetDateTime, 
    maturity: &OffsetDateTime, 
    calendar: &JointCalendar,
    conv: &BusinessDayConvention,
    freq: &PaymentFrequency, 
    fixing_gap_days: i64, 
    payment_gap_days: i64
) -> Result<Schedule> {
    if payment_gap_days < 0 || fixing_gap_days < 0 {
        // display all inputs. file and line are automatically filled by MyError
        let mut msg = String::from("payment_days and fixing_days should be non-negative\n");
        msg.push_str(&format!("effective_date: {:?}\n", effective_date));
        msg.push_str(&format!("maturity: {:?}\n", maturity));
        msg.push_str(&format!("calendar: {:?}\n", calendar.calendar_name()));
        msg.push_str(&format!("conv: {:?}\n", conv));
        msg.push_str(&format!("freq: {:?}\n", freq));
        msg.push_str(&format!("fixing_days: {:?}\n", fixing_gap_days));
        msg.push_str(&format!("payment_days: {:?}\n", payment_gap_days));
        return Err(anyhow!(msg));
    }
    
    let mut raw_start_date_vec: VecDeque<OffsetDateTime> = VecDeque::new();
    
    match forward_generation {
        true => {
            let mut raw_start_date = effective_date.clone();
            raw_start_date_vec.push_back(raw_start_date);
            let mut count = 1;
            while &raw_start_date < maturity {
                raw_start_date = add_period(&effective_date, &freq.to_string_with_multiple(count).as_str());
                count += 1;
                raw_start_date_vec.push_back(raw_start_date);
            }
        },
        false => {
            let mut raw_end_date = maturity.clone();
            raw_start_date_vec.push_front(raw_end_date);
            let mut count = 1;
            while &raw_end_date > effective_date {
                raw_end_date = sub_period(&maturity, &freq.to_string_with_multiple(count).as_str());
                count += 1;
                raw_start_date_vec.push_front(raw_end_date);
            }
        }
    }
    let calc_start_date_vec = raw_start_date_vec.iter().map(
        |x| calendar.adjust(x, conv)).collect::<Result<Vec<OffsetDateTime>>>()?;


    let schedule_length = calc_start_date_vec.len() - 1;
    let mut base_schedule_vec: Vec<BaseSchedule> = vec![];
    let mut fixing_date: OffsetDateTime;
    let mut calc_start_date: OffsetDateTime;
    let mut calc_end_date: OffsetDateTime;
    let mut payment_date: OffsetDateTime;
    for i in 0..schedule_length {
        if i == 0 {
            calc_start_date = effective_date.clone();
            calc_end_date = calc_start_date_vec[i + 1].clone();
            fixing_date = calendar.adjust(
                &(calc_start_date - Duration::days(fixing_gap_days)),
                    &BusinessDayConvention::Preceding,
                )?;
            payment_date = calendar.adjust(
                &(calc_end_date + Duration::days(payment_gap_days)), 
                conv
            )?;
        } 
        else if i == schedule_length - 1 {
            calc_start_date = calc_start_date_vec[i].clone();
            calc_end_date = maturity.clone();
            fixing_date = calendar.adjust(
                &(calc_start_date - Duration::days(fixing_gap_days)),
                &BusinessDayConvention::Preceding,
            )?;
            payment_date = maturity.clone();
        } 
        else {
            calc_start_date = calc_start_date_vec[i].clone();
            calc_end_date = calc_start_date_vec[i + 1].clone();
            fixing_date = calendar.adjust(
                &(calc_start_date - Duration::days(fixing_gap_days)),
                &BusinessDayConvention::Preceding,
            )?;
            payment_date = calendar.adjust(
                &(calc_end_date + Duration::days(payment_gap_days)),
                conv
            )?;
        }

        if calc_end_date <= calc_start_date {
            return Err(anyhow!(
                "({}:{}) {:?} <= {:?} in build_schedule\n\
                effective_date: {:?}\n\
                maturity: {:?}\n\
                calendar: {:?}\n\
                conv: {:?}\n\
                freq: {:?}\n\
                fixing_days: {:?}\n\
                payment_days: {:?}\n",
                file!(), line!(), calc_end_date.date(), calc_start_date.date(),
                effective_date, maturity, calendar, conv, freq, fixing_gap_days, payment_gap_days
            ));
        }

        base_schedule_vec.push(
            BaseSchedule::new(fixing_date, calc_start_date, calc_end_date, payment_date, None)
        );
    }

    let schedule = Schedule::new(base_schedule_vec);

    Ok(schedule)
}

#[cfg(test)] 
mod tests {
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use super::*;
    use time::macros::{datetime, date};
    use crate::time::calendar::Calendar;
    // make a test for
    // calendar = SouthKorea::new(SouthKoreaType::Settlement)
    // effective_date = date!(2023-08-02 16:00:00 +09:00)
    // frequency = PaymentFrequency::Quarterly
    // fix days = 1
    // pay days = 0
    // maturity = date!(2024-02-05 16:00:00 +09:00)
    // conv = ModifiedFollowing
    #[test]
    fn test_build_forward_schedule() -> Result<()> {
        let effective_date = datetime!(2023-01-31 16:30:00 +09:00);
        let maturity = datetime!(2024-07-31 16:30:00 +09:00);
        let cal = SouthKorea::new(SouthKoreaType::Settlement);
        let calendar = Calendar::SouthKorea(cal);
        let joint_calendar = JointCalendar::new(vec![calendar])?;
        let schedule = build_schedule(
            true,
            &effective_date, 
            &maturity, 
            &joint_calendar,
            &BusinessDayConvention::ModifiedFollowing,
            &PaymentFrequency::Quarterly,            
            1, 
            0
        ).expect("Failed to build schedule");

        for base_schedule in schedule.iter() {
            println!(
                "start = {:?}, end = {:?}" , 
                base_schedule.get_calc_start_date().date(), base_schedule.get_calc_end_date().date()
            );
        }

        // start = 2023-01-31, end = 2023-04-28
        // start = 2023-04-28, end = 2023-07-31
        // start = 2023-07-31, end = 2023-10-31
        // start = 2023-10-31, end = 2024-01-31
        // start = 2024-01-31, end = 2024-04-30
        // start = 2024-04-30, end = 2024-07-31
        assert_eq!(
            schedule[0].get_calc_start_date().date(),
            date!(2023-01-31),
        );

        assert_eq!(
            schedule[1].get_calc_start_date().date(),
            date!(2023-04-28),
        );

        assert_eq!(
            schedule[2].get_calc_start_date().date(),
            date!(2023-07-31),
        );

        assert_eq!(
            schedule[3].get_calc_start_date().date(),
            date!(2023-10-31),
        );

        assert_eq!(
            schedule[4].get_calc_start_date().date(),
            date!(2024-01-31),
        );

        assert_eq!(
            schedule[5].get_calc_start_date().date(),
            date!(2024-04-30),
        );


        Ok(())
    }
    
    #[test]
    fn test_build_backward_test() -> Result<()> {
        let effective_date = datetime!(2023-01-31 16:30:00 +09:00);
        let maturity = datetime!(2024-07-31 16:30:00 +09:00);
        let cal = SouthKorea::new(SouthKoreaType::Settlement);
        let calendar = Calendar::SouthKorea(cal);
        let joint_calendar = JointCalendar::new(vec![calendar])?;
        let schedule = build_schedule(
            false,
            &effective_date, 
            &maturity, 
            &joint_calendar,
            &BusinessDayConvention::ModifiedFollowing,
            &PaymentFrequency::Quarterly,            
            1, 
            0
        ).expect("Failed to build schedule");

        for base_schedule in schedule.iter() {
            println!(
                "start = {:?}, end = {:?}" , 
                base_schedule.get_calc_start_date().date(), base_schedule.get_calc_end_date().date()
            );
        }
        // start = 2023-01-31, end = 2023-04-28
        // start = 2023-04-28, end = 2023-07-31
        // start = 2023-07-31, end = 2023-10-31
        // start = 2023-10-31, end = 2024-01-31
        // start = 2024-01-31, end = 2024-04-30
        // start = 2024-04-30, end = 2024-07-31
        assert_eq!(
            schedule[0].get_calc_start_date().date(),
            date!(2023-01-31),
        );
        assert_eq!(
            schedule[1].get_calc_start_date().date(),
            date!(2023-04-28),
        );
        assert_eq!(
            schedule[2].get_calc_start_date().date(),
            date!(2023-07-31),
        );
        assert_eq!(
            schedule[3].get_calc_start_date().date(),
            date!(2023-10-31),
        );
        assert_eq!(
            schedule[4].get_calc_start_date().date(),
            date!(2024-01-31),
        );
        assert_eq!(
            schedule[5].get_calc_start_date().date(),
            date!(2024-04-30),
        );
        assert_eq!(
            schedule[5].get_calc_end_date().date(),
            date!(2024-07-31),
        );
        Ok(())
    }
    #[test]
    fn test_build_forward_test2() -> Result<()> {
        let effective_date = datetime!(2023-11-30 16:30:00 +09:00);
        let maturity = datetime!(2024-11-29 16:30:00 +09:00);
        let cal = SouthKorea::new(SouthKoreaType::Settlement);
        let calendar = Calendar::SouthKorea(cal);
        let joint_calendar = JointCalendar::new(vec![calendar])?;
        let schedule = build_schedule(
            true,
            &effective_date, 
            &maturity, 
            &joint_calendar,
            &BusinessDayConvention::ModifiedFollowing,
            &PaymentFrequency::Quarterly,            
            1, 
            0
        ).expect("Failed to build schedule");

        for base_schedule in schedule.iter() {
            println!(
                "start = {:?}, end = {:?}" , 
                base_schedule.get_calc_start_date().date(), base_schedule.get_calc_end_date().date()
            );
        }
        
        // start = 2023-11-30, end = 2024-02-29
        // start = 2024-02-29, end = 2024-05-30
        // start = 2024-05-30, end = 2024-08-30
        // start = 2024-08-30, end = 2024-11-29
        assert_eq!(
            schedule[0].get_calc_start_date().date(),
            date!(2023-11-30),
        );

        assert_eq!(
            schedule[1].get_calc_start_date().date(),
            date!(2024-02-29),
        );

        assert_eq!(
            schedule[2].get_calc_start_date().date(),
            date!(2024-05-30),
        );

        assert_eq!(
            schedule[3].get_calc_start_date().date(),
            date!(2024-08-30),
        );


        Ok(())
    }
}