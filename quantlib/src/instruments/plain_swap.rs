use crate::assets::currency::Currency;
use crate::definitions::{Integer, Real};
use crate::parameters::rate_index::RateIndex;
use crate::instruments::schedule::{self, Schedule};
use crate::parameters::zero_curve::ZeroCurve;
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::time::{
    jointcalendar::JointCalendar,
    calendar_trait::CalendarTrait,
};
use crate::instrument::InstrumentTrait;
use crate::data::history_data::CloseData;
use anyhow::{Result, Context, anyhow};
use serde::{Serialize, Deserialize};
use time::{OffsetDateTime, Duration};
use std::{
    rc::Rc,
    cell::RefCell,
    collections::HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash, Copy)]
pub enum PlainSwapType {
    IRS = 0,
    CRS = 1,
    FxSwap = 2,
    FxForward = 3,
    FxSpot = 4,
}

/// By the conbination of the attributes, we can represent
/// 1) IRS, OIS (initial and last swap amounts are all None)
/// 2) CRS (initial and last swap amounts are all Some(Real))
/// 3) FxSwap (schedule are empty)
/// 4) FxForward (schedule are empty and initial swap is None but last swap is Some(Real))
/// 5) FxSpot (same as FxForward but effective_date <= issue_date + 2 days)
/// Roughly in Fx or CRS case, fixed side is mostly KRW and Floating side is mostly USD 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainSwap {
    fixed_legs: Schedule,
    floating_legs: Schedule,
    fixed_rate: Option<Real>,
    rate_index: Option<RateIndex>,
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
    initial_fixed_side_endorsement: Option<Real>, 
    initial_floating_side_payment: Option<Real>, 
    last_fixed_side_payment: Option<Real>, 
    last_floating_side_endorsement: Option<Real>, 
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
    fixing_gap_days: i64,
    payment_gap_days: i64,
    //
    specific_type: PlainSwapType,
    name: String,
    code: String,
}

impl PlainSwap {
    pub fn new(
        fixed_legs: Schedule,
        floating_legs: Schedule,
        fixed_rate: Option<Real>,
        rate_index: Option<RateIndex>,
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
        initial_fixed_side_endorsement: Option<Real>, 
        initial_floating_side_payment: Option<Real>, 
        last_fixed_side_payment: Option<Real>, 
        last_floating_side_endorsement: Option<Real>, 
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
        fixing_gap_days: i64,
        payment_gap_days: i64,
        //
        name: String,
        code: String,
    ) -> Result<PlainSwap> {
        let mut specific_type: PlainSwapType;
        // IRS: initial and last swap amounts are all None but rate_index and fixed_rate are Some(Real)
        if initial_fixed_side_endorsement.is_none() &&
            initial_floating_side_payment.is_none() &&
            last_fixed_side_payment.is_none() &&
            last_floating_side_endorsement.is_none() &&
            rate_index.is_some() &&
            fixed_rate.is_some() {                
                specific_type = PlainSwapType::IRS;
        } 
        // CRS: initial, last swap amounts, rate_index, and fixed_rate are all Some(Real)
        else if initial_fixed_side_endorsement.is_some() &&
            initial_floating_side_payment.is_some() &&
            last_fixed_side_payment.is_some() &&
            last_floating_side_endorsement.is_some() &&
            rate_index.is_some() &&
            fixed_rate.is_some() &&
            fixed_leg_currency != floating_leg_currency {
                specific_type = PlainSwapType::CRS;
        }
        // FxSwap: initial and last swap amounts are all Some(Real).
        // In addition, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_some() &&
            initial_floating_side_payment.is_some() &&
            last_fixed_side_payment.is_some() &&
            last_floating_side_endorsement.is_some() &&
            fixed_legs.len() == 0 &&
            floating_legs.len() == 0 &&
            rate_index.is_none() &&
            fixed_rate.is_none() &&
            fixed_leg_currency != floating_leg_currency {
                specific_type = PlainSwapType::FxSwap;
        }
        // FxForward: initial swap amount is None but last swap amount is Some(Real)
        // Moreover, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_none() &&
            initial_floating_side_payment.is_none() &&
            last_fixed_side_payment.is_none() &&
            last_floating_side_endorsement.is_some() &&
            fixed_legs.len() == 0 &&
            floating_legs.len() == 0 &&
            rate_index.is_none() &&
            fixed_rate.is_none() &&
            fixed_leg_currency != floating_leg_currency {
                if effective_date.date() <= issue_date.date() + Duration::days(2) {
                    specific_type = PlainSwapType::FxSpot;
                } else {
                    specific_type = PlainSwapType::FxForward;
                }
        } 
        else {
            return Err(anyhow!(
                "({}:{}) Invalid PlainSwap type: {}({})",
                file!(), line!(), name, code
            ));
        }
        
        Ok(PlainSwap {
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            floating_compound_tenor,
            calendar,
            unit_notional,
            //
            issue_date,
            effective_date,
            maturity,
            //
            fixed_leg_currency,
            floating_leg_currency,
            //
            initial_fixed_side_endorsement,
            initial_floating_side_payment,
            last_fixed_side_payment,
            last_floating_side_endorsement,
            //
            fixed_daycounter,
            floating_daycounter,
            //
            fixed_busi_convention,
            floating_busi_convention,
            //
            fixed_frequency,
            floating_frequency,
            //
            fixing_gap_days,
            payment_gap_days,
            //
            specific_type,
            name,
            code,
        })
    }
    pub fn get_specific_type(&self) -> PlainSwapType {
        self.specific_type
    }
    /// construct IRS using PaymentFrequency, BusinessDayConvention, DayCountConvention
    /// without fixed/floating - legs given directly
    pub fn new_from_conventions(
        fixed_leg_currency: Currency,
        floating_leg_currency: Currency,
        //
        initial_fixed_side_endorsement: Option<Real>,
        initial_floating_side_payment: Option<Real>,
        last_fixed_side_payment: Option<Real>,
        last_floating_side_endorsement: Option<Real>,
        //
        unit_notional: Real,
        issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        maturity: OffsetDateTime,
        //
        fixed_first_coupon_date: Option<OffsetDateTime>,
        fixed_rate: Option<Real>,
        rate_index: Option<RateIndex>,
        floating_compound_tenor: Option<String>,
        fixed_daycounter: DayCountConvention,
        floating_daycounter: DayCountConvention,
        fixed_busi_convention: BusinessDayConvention,
        floating_busi_convention: BusinessDayConvention,
        fixed_frequency: PaymentFrequency,
        floating_frequency: PaymentFrequency,
        //
        fixing_gap_days: i64,
        payment_gap_days: i64,
        //
        calendar: JointCalendar,
        name: String,
        code: String,
    ) -> Result<PlainSwap> {
        let fixed_legs = schedule::build_schedule(
            &effective_date,
            None,
            &maturity,
            &calendar,
            &fixed_busi_convention,
            &fixed_frequency,
            fixing_gap_days,
            payment_gap_days,
        ).with_context(
            || anyhow!(
                "({}:{}) Failed to build fixed legs in IRS: {}({})", 
                file!(), line!(),
                &name, &code)
        )?;

        let floating_legs = schedule::build_schedule(
            &effective_date,
            fixed_first_coupon_date.as_ref(),
            &maturity,      
            &calendar,
            &floating_busi_convention,
            &floating_frequency,
            fixing_gap_days,
            payment_gap_days,
        ).with_context(
            || anyhow!(
                "({}:{}) Failed to build floating legs in IRS: {}({})", 
                file!(), line!(),
                &name, &code)
        )?;

        let mut specific_type: PlainSwapType;
        // IRS: initial and last swap amounts are all None but rate_index and fixed_rate are Some(Real)
        if initial_fixed_side_endorsement.is_none() &&
            initial_floating_side_payment.is_none() &&
            last_fixed_side_payment.is_none() &&
            last_floating_side_endorsement.is_none() &&
            rate_index.is_some() &&
            fixed_rate.is_some() {                
                specific_type = PlainSwapType::IRS;
        } 
        // CRS: initial, last swap amounts, rate_index, and fixed_rate are all Some(Real)
        else if initial_fixed_side_endorsement.is_some() &&
            initial_floating_side_payment.is_some() &&
            last_fixed_side_payment.is_some() &&
            last_floating_side_endorsement.is_some() &&
            rate_index.is_some() &&
            fixed_rate.is_some() &&
            fixed_leg_currency != floating_leg_currency {
                specific_type = PlainSwapType::CRS;
        }
        // FxSwap: initial and last swap amounts are all Some(Real).
        // In addition, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_some() &&
            initial_floating_side_payment.is_some() &&
            last_fixed_side_payment.is_some() &&
            last_floating_side_endorsement.is_some() &&
            fixed_legs.len() == 0 &&
            floating_legs.len() == 0 &&
            rate_index.is_none() &&
            fixed_rate.is_none() &&
            fixed_leg_currency != floating_leg_currency {
                specific_type = PlainSwapType::FxSwap;
        }
        // FxForward: initial swap amount is None but last swap amount is Some(Real)
        // Moreover, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_none() &&
            initial_floating_side_payment.is_none() &&
            last_fixed_side_payment.is_none() &&
            last_floating_side_endorsement.is_some() &&
            fixed_legs.len() == 0 &&
            floating_legs.len() == 0 &&
            rate_index.is_none() &&
            fixed_rate.is_none() &&
            fixed_leg_currency != floating_leg_currency {
                if effective_date.date() <= issue_date.date() + Duration::days(2) {
                    specific_type = PlainSwapType::FxSpot;
                } else {
                    specific_type = PlainSwapType::FxForward;
                }
        } 
        else {
            return Err(anyhow!(
                "({}:{}) Invalid PlainSwap type: {}({})",
                file!(), line!(), name, code
            ));
        }

        Ok(PlainSwap {
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            floating_compound_tenor,
            calendar,
            unit_notional,
            //
            issue_date,
            effective_date,
            maturity,
            //
            fixed_leg_currency,
            floating_leg_currency,
            //
            initial_fixed_side_endorsement,
            initial_floating_side_payment,
            last_fixed_side_payment,
            last_floating_side_endorsement,
            //
            fixed_daycounter,
            floating_daycounter,
            //
            fixed_busi_convention,
            floating_busi_convention,
            //
            fixed_frequency,
            floating_frequency,
            //
            fixing_gap_days,
            payment_gap_days,
            //
            specific_type,
            name,
            code,
        })
    }

    fn get_fixed_cashflows(
        &self, pricing_date: &OffsetDateTime
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();

        if self.effective_date.date() >= pricing_date.date() &&
        self.initial_fixed_side_endorsement.is_some() {
            res.insert(self.effective_date.clone(), self.initial_fixed_side_endorsement.unwrap());
        }

        if self.maturity.date() >= pricing_date.date() &&
        self.last_fixed_side_payment.is_some(){
            res.insert(self.maturity.clone(), -self.last_fixed_side_payment.unwrap());
        }

        if self.fixed_rate.is_none() || self.fixed_legs.len() == 0 {
            return Ok(res);
        }

        let fixed_rate = self.fixed_rate.unwrap();
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

            let amount = fixed_rate * frac;
            res.insert(payment_date.clone(), amount);
        }

        Ok(res)
    }

    fn get_floating_cashflows(
        &self, 
        pricing_date: &OffsetDateTime, 
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<CloseData>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        if self.effective_date.date() >= pricing_date.date() && 
        self.initial_floating_side_payment.is_some() {
            res.insert(self.effective_date.clone(), - self.initial_floating_side_payment.unwrap());
        }
        if self.maturity.date() >= pricing_date.date() &&
        self.last_floating_side_endorsement.is_some() {
            res.insert(self.maturity.clone(), - self.last_floating_side_endorsement.unwrap());
        }

        if self.rate_index.is_none() || self.floating_legs.len() == 0 {
            return Ok(res);
        }

        let rate_index = self.rate_index.as_ref().unwrap();
        for base_schedule in self.floating_legs.iter() {
            let payment_date = base_schedule.get_payment_date();
            if payment_date.date() < pricing_date.date() {
                continue;
            }

            let amount = rate_index.get_coupon_amount(
                &base_schedule,
                None,
                forward_curve.clone().unwrap(),
                past_fixing_data.clone().unwrap_or(Rc::new(CloseData::default())),
                pricing_date,
                self.floating_compound_tenor.as_ref(),
                &self.calendar,
                &self.floating_daycounter,
                self.fixing_gap_days,
            )?;
            res.insert(payment_date.clone(), amount);
        }

        Ok(res)
    }
}

impl InstrumentTrait for PlainSwap {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_code(&self) -> &String {
        &self.code
    }

    fn get_currency(&self) -> &Currency {
        &self.fixed_leg_currency
    }

    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        Some(&self.maturity)
    }

    fn get_unit_notional(&self) -> Real {
        self.unit_notional
    }

    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        Ok(self.rate_index.as_ref())
    }

    fn get_type_name(&self) -> &'static str {
        "PlainSwap"
    }

    fn get_fixed_leg_currency(&self) -> Result<&Currency> {
        Ok(&self.fixed_leg_currency)
    }

    fn get_floating_leg_currency(&self) -> Result<&Currency> {
        Ok(&self.floating_leg_currency)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assets::currency::Currency,
        time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
        time::calendars::{
            southkorea::{SouthKorea, SouthKoreaType},
            unitedstates::{UnitedStates, UnitedStatesType},
        },
        time::calendar::Calendar,
        time::jointcalendar::JointCalendar,
        parameters::rate_index::RateIndex,
        enums::RateIndexCode,
    };
    use rand::seq::index::IndexVecIntoIter;
    use time::macros::datetime;
    use anyhow::Result;

    #[test]
    fn test_fx_swap() -> Result<()> {
        let fixed_currency = Currency::KRW;
        let floating_currency = Currency::USD;
        let unit_notional = 10_000_000.0;
        let issue_date = datetime!(2024-01-02 16:30:00 +09:00);
        let maturity = datetime!(2025-01-02 16:30:00 +09:00);
        let fixed_rate = 0.01;
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let us = Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Settlement));
        let calendar = JointCalendar::new(vec![sk, us])?;
        
        let fixing_gap_days = 0;
        let payment_gap_days = 0;

        let first_fx_rate = 1300.0;
        let last_fx_rate = 1280.0;

        let initial_fixed_side_endorsement = Some(unit_notional * first_fx_rate);
        let initial_floating_side_payment = Some(unit_notional);
        let last_fixed_side_payment = Some(unit_notional * last_fx_rate);
        let last_floating_side_endorsement = Some(unit_notional);
        
        let fx_swap = PlainSwap::new(
            Schedule::default(),
            Schedule::default(),
            None,
            None,
            None,
            calendar,
            1.0,
            issue_date.clone(),
            issue_date.clone(),
            maturity.clone(),
            fixed_currency,
            floating_currency,
            initial_fixed_side_endorsement,
            initial_floating_side_payment,
            last_fixed_side_payment,
            last_floating_side_endorsement,
            //
            DayCountConvention::Dummy,
            DayCountConvention::Dummy,
            //
            BusinessDayConvention::Dummy,
            BusinessDayConvention::Dummy,
            //
            PaymentFrequency::None,
            PaymentFrequency::None,
            //
            fixing_gap_days,
            payment_gap_days,
            //
            "MockFxSwap".to_string(),
            "MockCode".to_string(),
        )?;

        let fixed_cashflows = fx_swap.get_fixed_cashflows(&issue_date)?;
        let floating_cashflows = fx_swap.get_floating_cashflows(&issue_date, None, None)?;

        println!("fixed_cashflows: {:?}", fixed_cashflows);
        println!("floating_cashflows: {:?}", floating_cashflows);
        Ok(())
    }

}