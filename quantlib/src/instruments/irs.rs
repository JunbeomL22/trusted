use crate::assets::currency::Currency;
use crate::definitions::{Integer, Real};
use crate::parameters::rate_index::RateIndex;
use crate::instruments::schedule::{self, Schedule};
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::time::{
    jointcalendar::JointCalendar,
    calendar_trait::CalendarTrait,
};
use crate::instrument::InstrumentTrait;
use crate::data::history_data::CloseData;
use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

/// This includes the case of CRS
/// In crs case, fixed_leg_currency != floating_leg_currency and 
/// the following fields are not zero: 
/// initial_fixed_side_payment, initial_floating_side_endorsement,
/// last_fixed_side_endorsement, last_floating_side_payment.\n
/// In plain IRS case, fixed_leg_currency == floating_leg_currency and
/// the initial and last swaps of notional amounts are zero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IRS {
    fixed_legs: Schedule,
    floating_legs: Schedule,
    fixed_rate: Real,
    rate_index: RateIndex,
    floating_compound_tenor: Option<String>,
    calendar: JointCalendar,
    unit_notional: Real,
    //
    issue_date: OffsetDateTime,
    effective_date: OffsetDateTime,
    maturity: OffsetDateTime,
    //
    fixed_leg_currency: Currency,
    floating_leg_currency: Currency,
    //
    initial_fixed_side_payment: Real, // 0.0 in plain IRS case
    initial_floating_side_endorsement: Real, // 0.0 in plain IRS case
    last_fixed_side_endorsement: Real, // 0.0 in plain IRS case
    last_floating_side_payment: Real, // 0.0 in plain IRS case
    //
    fixed_daycounter: DayCountConvention,
    floating_daycounter: DayCountConvention,
    //
    fixed_busi_convention: BusinessDayConvention,
    floating_busi_convention: BusinessDayConvention,
    //
    fixed_frequency: PaymentFrequency,
    floating_frequency: PaymentFrequency,
    //
    fixing_days: Integer,
    payment_days: Integer,
    //
    name: String,
    code: String,
}

impl IRS {
    pub fn new(
        fixed_legs: Schedule,
        floating_legs: Schedule,
        fixed_rate: Real,
        rate_index: RateIndex,
        floating_compound_tenor: Option<String>,
        calendar: JointCalendar,
        currency: Currency,
        unit_notional: Real,
        //
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        maturity: OffsetDateTime,
        //
        fixed_daycounter: DayCountConvention,
        floating_daycounter: DayCountConvention,
        //
        fixed_busi_convention: BusinessDayConvention,
        floating_busi_convention: BusinessDayConvention,
        //
        fixed_frequency: PaymentFrequency,
        floating_frequency: PaymentFrequency,
        //
        fixing_days: Integer,
        payment_days: Integer,
        //
        name: String,
        code: String,
    ) -> IRS {
        IRS {
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            floating_compound_tenor,
            calendar,
            currency,
            unit_notional,
            issue_date,
            effective_date,
            maturity,
            fixed_daycounter,
            floating_daycounter,
            fixed_busi_convention,
            floating_busi_convention,
            fixed_frequency,
            floating_frequency,
            fixing_days,
            payment_days,
            name,
            code,
        }
    }
    /// construct IRS using PaymentFrequency, BusinessDayConvention, DayCountConvention
    /// without fixed/floating - legs given directly
    pub fn new_from_conventions(
        currency: Currency,
        unit_notional: Real,
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        maturity: OffsetDateTime,
        //
        fixed_first_coupon_date: Option<OffsetDateTime>,
        fixed_rate: Real,
        rate_index: RateIndex,
        floating_compound_tenor: Option<String>,
        fixed_daycounter: DayCountConvention,
        floating_daycounter: DayCountConvention,
        fixed_busi_convention: BusinessDayConvention,
        floating_busi_convention: BusinessDayConvention,
        fixed_frequency: PaymentFrequency,
        floating_frequency: PaymentFrequency,
        fixing_days: Integer,
        payment_days: Integer,
        calendar: JointCalendar,
        name: String,
        code: String,
    ) -> Result<IRS> {
        let fixed_legs = schedule::build_schedule(
            &effective_date,
            None,
            &maturity,
            &calendar,
            &fixed_busi_convention,
            &fixed_frequency,
            fixing_days as i64,
            payment_days as i64,
        ).with_context(
            || anyhow!("Failed to build fixed legs in IRS: {}({})", &name, &code)
        )?;

        let floating_legs = schedule::build_schedule(
            &effective_date,
            fixed_first_coupon_date.as_ref(),
            &maturity,      
            &calendar,
            &floating_busi_convention,
            &floating_frequency,
            fixing_days as i64,
            payment_days as i64,
        ).with_context(
            || anyhow!("Failed to build floating legs in IRS: {}({})", &name, &code)
        )?;

        Ok(IRS {
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            floating_compound_tenor,
            calendar,
            currency,
            unit_notional,
            issue_date,
            effective_date,
            maturity,
            fixed_daycounter,
            floating_daycounter,
            fixed_busi_convention,
            floating_busi_convention,
            fixed_frequency,
            floating_frequency,
            fixing_days,
            payment_days,
            name,
            code,
        })
    }

    fn get_fixed_cashflows(
        &self, pricing_date: &OffsetDateTime
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        let mut frac: Real;
        
        for base_schedule in self.fixed_legs.iter() {
            let payment_date = base_schedule.get_payment_date();
            if payment_date.date() < pricing_date.date() {
                continue;
            }

            frac = self.calendar.year_fraction(
                &base_schedule.get_calc_start_date(),
                &base_schedule.get_calc_end_date(),
                &self.fixed_daycounter
            )?;

            let amount = self.fixed_rate * frac;
            res.insert(payment_date.clone(), amount);
        }
        Ok(res)
    }

    fn get_floating_cashflow(
        &self, pricing_date: &OffsetDateTime, 
        forward_curve: Rc<RefCell<HashMap<OffsetDateTime, Real>>>,
        past_fixing_data: Rc<RefCell<CloseData>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        let mut frac: Real;
        let mut disc_factor: Real;
        let mut fixing_date: OffsetDateTime;
        let mut fixing_rate: Real;
        let mut fixing_data: Real;
        let mut fixing_rate_index: RateIndex;
        let mut fixing_rate_index_name: String;
        let mut fixing_rate_index_code: String;

        for base_schedule in self.floating_legs.iter() {
            let payment_date = base_schedule.get_payment_date();
            if payment_date.date() < pricing_date.date() {
                continue;
            }

            frac = self.calendar.year_fraction(
                &base_schedule.get_calc_start_date(),
                &base_schedule.get_calc_end_date(),
                &self.floating_daycounter
            )?;

            fixing_date = self.calendar.adjust_date(
                &payment_date,
                &self.rate_index.get_calendar(),
                &self.rate_index.get_business_day_convention()
            )?;

            fixing_rate_index_name = self.rate_index.get_name();
            fixing_rate_index_code = self.rate_index.get_code();
            fixing_rate_index = RateIndex::new(
                fixing_rate_index_name.clone(),
                self.currency.clone(),
                self.rate_index.get_rate_index_code(),
                fixing_rate_index_code.clone()
            )?;

            fixing_data = past_fixing_data.borrow().get(&fixing_date)
                .ok_or_else(
                    || anyhow!(
                        "Failed to get fixing data of {} on {}",
                        fixing_rate_index_name,
                        fixing_date
                    )
                )?;

            fixing_rate = fixing_data;
            disc_factor = 1.0 / (1.0 + fixing_rate).powf(frac);
            res.insert(payment_date.clone(), disc_factor);
        }
        Ok(res)
    }
}

impl InstrumentTrait for IRS {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.currency
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        Ok(Some(&self.rate_index))
    }

    fn get_type_name(&self) -> &'static str {
        "IRS"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assets::currency::Currency,
        time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
        time::calendars::southkorea::{SouthKorea, SouthKoreaType},
        time::calendar::Calendar,
        time::jointcalendar::JointCalendar,
        parameters::rate_index::RateIndex,
        enums::RateIndexCode,
    };
    use time::macros::datetime;
    use anyhow::Result;

    #[test]
    fn test_new_from_convention() -> Result<()> {
        let currency = Currency::KRW;
        let unit_notional = 100.0;
        let issue_date = datetime!(2021-01-01 00:00:00 +09:00);
        let maturity = datetime!(2021-12-31 00:00:00 +09:00);
        let fixed_rate = 0.01;
        let sk = JointCalendar::new(
            vec![Calendar::SouthKorea(
                SouthKorea::new(SouthKoreaType::Settlement)
                )]
            )?;

        let rate_index = RateIndex::new(
            String::from("91D"),
            Currency::KRW,
            RateIndexCode::CD,
            "CD91".to_string(),
        )?;

        let fixing_days = 2;
        let payment_days = 0;

        let sk = JointCalendar::new(
            vec![Calendar::SouthKorea(
                SouthKorea::new(SouthKoreaType::Settlement)
                )]
            )?;
        
        let irs = IRS::new_from_conventions(
            currency,
            unit_notional,
            issue_date.clone(),
            issue_date.clone(),
            maturity,
            None,
            fixed_rate,
            rate_index.clone(),
            None,
            DayCountConvention::Actual365Fixed,
            DayCountConvention::Actual365Fixed,
            BusinessDayConvention::ModifiedFollowing,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            PaymentFrequency::Quarterly,
            fixing_days,
            payment_days,
            sk,
            "IRS".to_string(),
            "IRS".to_string(),
        )?;

        println!("{:?}", irs);
        assert_eq!(currency, *irs.get_currency());
        assert_eq!(unit_notional, irs.get_unit_notional());
        assert_eq!(maturity, irs.get_maturity().unwrap().clone());

        Ok(())
    }

}