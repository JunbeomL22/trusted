use crate::currency::{Currency, FxCode};
use crate::definitions::Real;
use crate::instrument::InstrumentTrait;
use crate::instruments::schedule::{self, Schedule};
use crate::parameters::past_price::DailyClosePrice;
use crate::parameters::rate_index::RateIndex;
use crate::parameters::zero_curve::ZeroCurve;
use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
use crate::time::{calendar_trait::CalendarTrait, jointcalendar::JointCalendar};
use crate::InstInfo;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Hash, Copy)]
pub enum PlainSwapType {
    IRS = 0,
    CRS = 1,
    FxSwap = 2,
    FxForward = 3,
    FxSpot = 4,
}

impl PlainSwapType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            PlainSwapType::IRS => "IRS",
            PlainSwapType::CRS => "CRS",
            PlainSwapType::FxSwap => "FxSwap",
            PlainSwapType::FxForward => "FxForward",
            PlainSwapType::FxSpot => "FxSpot",
        }
    }
}
/// By the conbination of the attributes, we can represent
/// 1) IRS, OIS (initial and last swap amounts are all None)
/// 2) CRS (initial and last swap amounts are all Some(Real))
/// 3) FxSwap (schedule are empty)
/// 4) FxForward (schedule are empty and initial swap is None but last swap is Some(Real))
/// 5) FxSpot (same as FxForward but effective_date <= issue_date + 2 days)
/// Roughly in Fx or CRS case, fixed side is mostly KRW and Floating side is mostly USD
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlainSwap {
    pub inst_info: InstInfo,
    //
    pub fixed_legs: Schedule,
    pub floating_legs: Schedule,
    pub fixed_rate: Option<Real>,
    pub rate_index: Option<RateIndex>,
    pub floating_compound_tenor: Option<String>,
    pub calendar: JointCalendar,
    //unit_notional: Real,
    //
    //issue_date: OffsetDateTime,
    pub effective_date: OffsetDateTime,
    //maturity: OffsetDateTime,
    //
    pub fixed_leg_currency: Currency,
    pub floating_leg_currency: Currency,
    pub floating_to_fixed_fxcode: Option<FxCode>,
    //
    pub initial_fixed_side_endorsement: Option<Real>,
    pub initial_floating_side_payment: Option<Real>,
    pub last_fixed_side_payment: Option<Real>,
    pub last_floating_side_endorsement: Option<Real>,
    //
    pub fixed_daycounter: DayCountConvention,
    pub floating_daycounter: DayCountConvention,
    //
    pub fixed_busi_convention: BusinessDayConvention,
    pub floating_busi_convention: BusinessDayConvention,
    //
    pub fixed_frequency: PaymentFrequency,
    pub floating_frequency: PaymentFrequency,
    //
    pub fixing_gap_days: i64,
    pub payment_gap_days: i64,
    //
    pub specific_type: PlainSwapType,
    //name: String,
    //code: String,
}

impl PlainSwap {
    /// By the conbination of the attributes, we can represent
    /// 1) IRS, OIS (initial and last swap amounts are all None)
    /// 2) CRS (initial and last swap amounts are all Some(Real))
    /// 3) FxSwap (schedule are empty)
    /// 4) FxForward (schedule are empty and initial swap is None but last swap is Some(Real))
    /// 5) FxSpot (same as FxForward but effective_date <= issue_date + 2 days)
    /// Roughly in Fx or CRS case, fixed side is mostly KRW and Floating side is mostly USD
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        inst_info: InstInfo,
        //
        fixed_legs: Schedule,
        floating_legs: Schedule,
        fixed_rate: Option<Real>,
        rate_index: Option<RateIndex>,
        floating_compound_tenor: Option<String>,
        calendar: JointCalendar,
        //unit_notional: Real,
        //
        //issue_date: OffsetDateTime,
        effective_date: OffsetDateTime,
        //maturity: OffsetDateTime,
        //
        //fixed_leg_currency: Currency,
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
        //name: String,
        //code: String,
    ) -> Result<PlainSwap> {
        let specific_type: PlainSwapType;
        let issue_date = inst_info.issue_date.unwrap();
        let maturity = inst_info.maturity.unwrap();
        let fixed_leg_currency = inst_info.currency;
        // IRS: initial and last swap amounts are all None but rate_index and fixed_rate are Some(Real)
        if initial_fixed_side_endorsement.is_none()
            && initial_floating_side_payment.is_none()
            && last_fixed_side_payment.is_none()
            && last_floating_side_endorsement.is_none()
            && rate_index.is_some()
            && fixed_rate.is_some()
            && fixed_leg_currency == floating_leg_currency
        {
            specific_type = PlainSwapType::IRS;
        }
        // CRS: initial, last swap amounts, rate_index, and fixed_rate are all Some(Real)
        else if initial_fixed_side_endorsement.is_some()
            && initial_floating_side_payment.is_some()
            && last_fixed_side_payment.is_some()
            && last_floating_side_endorsement.is_some()
            && rate_index.is_some()
            && fixed_rate.is_some()
            && fixed_leg_currency != floating_leg_currency
        {
            specific_type = PlainSwapType::CRS;
        }
        // FxSwap: initial and last swap amounts are all Some(Real).
        // In addition, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_some()
            && initial_floating_side_payment.is_some()
            && last_fixed_side_payment.is_some()
            && last_floating_side_endorsement.is_some()
            && fixed_legs.is_empty()
            && floating_legs.is_empty()
            && rate_index.is_none()
            && fixed_rate.is_none()
            && fixed_leg_currency != floating_leg_currency
        {
            specific_type = PlainSwapType::FxSwap;
        }
        // FxForward: initial swap amount is None but last swap amount is Some(Real)
        // Moreover, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_none()
            && initial_floating_side_payment.is_none()
            && last_fixed_side_payment.is_some()
            && last_floating_side_endorsement.is_some()
            && fixed_legs.is_empty()
            && floating_legs.is_empty()
            && rate_index.is_none()
            && fixed_rate.is_none()
            && fixed_leg_currency != floating_leg_currency
        {
            if maturity.date() <= issue_date.date() + Duration::days(2) {
                specific_type = PlainSwapType::FxSpot;
            } else {
                specific_type = PlainSwapType::FxForward;
            }
        } else {
            let err = || anyhow!(
                "({}:{}) Invalid PlainSwap type: {:?} \n\
                initial_fixed_side_endorsement: {:?}\n\
                initial_floating_side_payment: {:?}\n\
                last_fixed_side_payment: {:?}\n\
                last_floating_side_endorsement: {:?}\n\
                rate_index: {:?}\n\
                fixed_rate: {:?}",
                file!(),
                line!(),
                inst_info.id,
                initial_fixed_side_endorsement,
                initial_floating_side_payment,
                last_fixed_side_payment,
                last_floating_side_endorsement,
                rate_index,
                fixed_rate
            );

            return Err(err());
        }

        let floating_to_fixed_fxcode = match fixed_leg_currency == floating_leg_currency {
            true => None,
            false => Some(FxCode::new(floating_leg_currency, fixed_leg_currency)),
        };

        Ok(PlainSwap {
            inst_info,
            //
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            floating_compound_tenor,
            calendar,
            //
            effective_date,
            //
            fixed_leg_currency,
            floating_leg_currency,
            floating_to_fixed_fxcode,
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
        })
    }
    /// By the conbination of the attributes, we can represent
    /// 1) IRS, OIS (initial and last swap amounts are all None)
    /// 2) CRS (initial and last swap amounts are all Some(Real))
    /// 3) FxSwap (schedule are empty)
    /// 4) FxForward (schedule are empty and initial swap is None but last swap is Some(Real))
    /// 5) FxSpot (same as FxForward but effective_date <= issue_date + 2 days)
    /// Roughly in Fx or CRS case, fixed side is mostly KRW and Floating side is mostly USD
    /// construct IRS using PaymentFrequency, BusinessDayConvention, DayCountConvention
    /// without schedule given directly
    //#[allow(clippy::too_many_arguments)]
    pub fn new_from_conventions(
        inst_info: InstInfo,    
        //
        //fixed_leg_currency: Currency,
        floating_leg_currency: Currency,
        //
        initial_fixed_side_endorsement: Option<Real>,
        initial_floating_side_payment: Option<Real>,
        last_fixed_side_payment: Option<Real>,
        last_floating_side_endorsement: Option<Real>,
        //
        effective_date: OffsetDateTime,
        //
        fixed_rate: Option<Real>,
        rate_index: Option<RateIndex>,
        floating_compound_tenor: Option<String>,
        //
        forward_generation: bool,
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
    ) -> Result<PlainSwap> {
        let maturity = inst_info.get_maturity().unwrap();
        let issue_date = inst_info.get_issue_date().unwrap();
        let fixed_leg_currency = inst_info.currency;
        let fixed_legs = schedule::build_schedule(
            forward_generation,
            &effective_date,
            maturity,
            &calendar,
            &fixed_busi_convention,
            &fixed_frequency,
            fixing_gap_days,
            payment_gap_days,
        )
        .with_context(|| {
            anyhow!(
                "({}:{}) Failed to build fixed legs in IRS: ({:?})",
                file!(),
                line!(),
                inst_info.id
            )
        })?;

        let floating_legs = schedule::build_schedule(
            forward_generation,
            &effective_date,
            &maturity,
            &calendar,
            &floating_busi_convention,
            &floating_frequency,
            fixing_gap_days,
            payment_gap_days,
        )
        .with_context(|| {
            anyhow!(
                "({}:{}) Failed to build floating legs in IRS: ({:?}) ",
                file!(),
                line!(),
                inst_info.id
            )
        })?;

        let specific_type: PlainSwapType;
        // IRS: initial and last swap amounts are all None but rate_index and fixed_rate are Some(Real)
        if initial_fixed_side_endorsement.is_none()
            && initial_floating_side_payment.is_none()
            && last_fixed_side_payment.is_none()
            && last_floating_side_endorsement.is_none()
            && rate_index.is_some()
            && fixed_rate.is_some()
        {
            specific_type = PlainSwapType::IRS;
        }
        // CRS: initial, last swap amounts, rate_index, and fixed_rate are all Some(Real)
        else if initial_fixed_side_endorsement.is_some()
            && initial_floating_side_payment.is_some()
            && last_fixed_side_payment.is_some()
            && last_floating_side_endorsement.is_some()
            && rate_index.is_some()
            && fixed_rate.is_some()
            && fixed_leg_currency != floating_leg_currency
        {
            specific_type = PlainSwapType::CRS;
        }
        // FxSwap: initial and last swap amounts are all Some(Real).
        // In addition, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_some()
            && initial_floating_side_payment.is_some()
            && last_fixed_side_payment.is_some()
            && last_floating_side_endorsement.is_some()
            && fixed_legs.is_empty()
            && floating_legs.is_empty()
            && rate_index.is_none()
            && fixed_rate.is_none()
            && fixed_leg_currency != floating_leg_currency
        {
            specific_type = PlainSwapType::FxSwap;
        }
        // FxForward: initial swap amount is None but last swap amount is Some(Real)
        // Moreover, schedules are empty and rate_index and fixed_rate are None
        else if initial_fixed_side_endorsement.is_none()
            && initial_floating_side_payment.is_none()
            && last_fixed_side_payment.is_some()
            && last_floating_side_endorsement.is_some()
            && fixed_legs.is_empty()
            && floating_legs.is_empty()
            && rate_index.is_none()
            && fixed_rate.is_none()
            && fixed_leg_currency != floating_leg_currency
        {
            if maturity.date() <= issue_date.date() + Duration::days(2) {
                specific_type = PlainSwapType::FxSpot;
            } else {
                specific_type = PlainSwapType::FxForward;
            }
        } else {
            return Err(anyhow!(
                "({}:{}) Invalid PlainSwap type: ({:?})\n\
                initial_fixed_side_endorsement: {:?}\n\
                initial_floating_side_payment: {:?}\n\
                last_fixed_side_payment: {:?}\n\
                last_floating_side_endorsement: {:?}\n\
                rate_index: {:?}\n\
                fixed_rate: {:?}",
                file!(),
                line!(),
                inst_info.id,
                initial_fixed_side_endorsement,
                initial_floating_side_payment,
                last_fixed_side_payment,
                last_floating_side_endorsement,
                rate_index,
                fixed_rate
            ));
        }

        let floating_to_fixed_fxcode = match fixed_leg_currency == floating_leg_currency {
            true => None,
            false => Some(FxCode::new(floating_leg_currency, fixed_leg_currency)),
        };
        Ok(PlainSwap {
            inst_info,
            //
            fixed_legs,
            floating_legs,
            fixed_rate,
            rate_index,
            floating_compound_tenor,
            calendar,
            //
            effective_date,
            //
            fixed_leg_currency,
            floating_leg_currency,
            floating_to_fixed_fxcode,
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
        })
    }
}

impl InstrumentTrait for PlainSwap {
    fn get_inst_info(&self) ->  &InstInfo {
        &self.inst_info
    }

    fn get_fixed_cashflows(
        &self,
        pricing_date: &OffsetDateTime,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        let initial_value = self.initial_fixed_side_endorsement.unwrap_or(1.0);

        if self.effective_date.date() >= pricing_date.date()
            && self.initial_fixed_side_endorsement.is_some()
        {
            res.insert(self.effective_date, initial_value);
        }

        let maturity = self.get_maturity().unwrap();
        if maturity.date() >= pricing_date.date() && self.last_fixed_side_payment.is_some() {
            res.insert(maturity.clone(), -self.last_fixed_side_payment.unwrap());
        }

        if self.fixed_rate.is_none() || self.fixed_legs.is_empty() {
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
                base_schedule.get_calc_start_date(),
                base_schedule.get_calc_end_date(),
                &self.fixed_daycounter,
            )?;

            // an initial amount for fixed_leg is initially endorsed so it is a payment
            let amount = -fixed_rate * frac * initial_value;

            res.entry(*payment_date)
                .and_modify(|e| *e += amount)
                .or_insert(amount);
        }

        Ok(res)
    }

    fn get_floating_cashflows(
        &self,
        pricing_date: &OffsetDateTime,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<DailyClosePrice>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        let mut res = HashMap::new();
        let mut initial_value = 1.0;
        if self.effective_date.date() >= pricing_date.date()
            && self.initial_floating_side_payment.is_some()
        {
            initial_value = self.initial_floating_side_payment.unwrap();
            res.insert(self.effective_date, -initial_value);
        }

        let maturity = self.get_maturity().unwrap();
        if maturity.date() >= pricing_date.date()
            && self.last_floating_side_endorsement.is_some()
        {
            res.insert(maturity.clone(), self.last_floating_side_endorsement.unwrap());
        }

        if self.rate_index.is_none() || self.floating_legs.is_empty() {
            return Ok(res);
        }

        let rate_index = self.rate_index.as_ref().unwrap();
        for base_schedule in self.floating_legs.iter() {
            let payment_date = base_schedule.get_payment_date();
            if payment_date.date() < pricing_date.date() {
                continue;
            }

            let amount = rate_index.get_coupon_amount(
                base_schedule,
                None,
                forward_curve.clone().unwrap(),
                past_fixing_data
                    .clone()
                    .unwrap_or(Rc::new(DailyClosePrice::default())),
                pricing_date,
                self.floating_compound_tenor.as_ref(),
                &self.calendar,
                &self.floating_daycounter,
                self.fixing_gap_days,
            )? * initial_value;

            res.entry(*payment_date)
                .and_modify(|e| *e += amount)
                .or_insert(amount);
            //res.insert(payment_date.clone(), amount);
        }

        Ok(res)
    }

    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        Ok(self.rate_index.as_ref())
    }

    fn get_type_name(&self) -> &'static str {
        //"PlainSwap"
        self.specific_type.as_str()
    }

    fn get_fixed_leg_currency(&self) -> Result<Currency> {
        Ok(self.fixed_leg_currency)
    }

    fn get_floating_leg_currency(&self) -> Result<Currency> {
        Ok(self.floating_leg_currency)
    }

    fn get_specific_plain_swap_type(&self) -> Result<PlainSwapType> {
        Ok(self.specific_type)
    }

    fn get_floating_to_fixed_fxcode(&self) -> Result<Option<FxCode>> {
        Ok(self.floating_to_fixed_fxcode)
    }

    fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> {
        let mut res = Vec::new();
        if let Some(floating_to_fixed_fxcode) = self.floating_to_fixed_fxcode.as_ref() {
            res.push(*floating_to_fixed_fxcode);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vector_data::VectorData;
    use crate::evaluation_date::EvaluationDate;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::{
        currency::Currency,
        parameters::rate_index::RateIndex,
        time::{
            calendar::Calendar,
            calendars::{
                southkorea::{SouthKorea, SouthKoreaType},
                unitedstates::{UnitedStates, UnitedStatesType},
            },
            conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency},
            jointcalendar::JointCalendar,
        },
    };
    use anyhow::Result;
    use ndarray::array;
    use std::{cell::RefCell, rc::Rc};
    use time::macros::datetime;

    #[test]
    fn test_crs() -> Result<()> {
        let fixed_currency = Currency::KRW;
        let floating_currency = Currency::USD;
        let unit_notional = 10_000_000.0;
        let issue_date = datetime!(2024-01-02 16:30:00 +09:00);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(issue_date.clone())));
        let effective_date = datetime!(2024-01-03 16:30:00 +09:00);
        let maturity = datetime!(2025-01-03 16:30:00 +09:00);
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let us = Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Settlement));
        let calendar = JointCalendar::new(vec![sk, us])?;

        let fixing_gap_days = 1;
        let payment_gap_days = 0;

        let fixed_rate = 0.04;
        let fx_rate = 1_330.0;
        let rate_index = RateIndex::new(
            String::from("3M"),
            Currency::USD,
            String::from("USD Libor 3M"),
            String::from("USD Libor 3M"), // this is just a mock code
        )?;

        let initial_fixed_side_endorsement = Some(fx_rate);
        let initial_floating_side_payment = Some(1.0);
        let last_fixed_side_payment = Some(fx_rate);
        let last_floating_side_endorsement = Some(1.0);

        let instid = crate::InstId::new(
            crate::Symbol::Ticker(crate::Ticker::new(b"PlainSwap:XXX")?),
            crate::Venue::KRX,
        );

        let inst_info = crate::InstInfo::new(
            instid,
            "MockCRS".to_string(),
            crate::InstType::PlainSwap,
            Currency::KRW,
            10_000_000.0,
            Some(issue_date.clone()),
            Some(maturity.clone()),
            crate::AccountingLevel::L2,
        );
        
        let crs = PlainSwap::new_from_conventions(
            inst_info.clone(),
            floating_currency,
            //
            initial_fixed_side_endorsement,
            initial_floating_side_payment,
            last_fixed_side_payment,
            last_floating_side_endorsement,
            //
            effective_date.clone(),
            //
            Some(fixed_rate),
            Some(rate_index),
            None,
            //
            true,
            DayCountConvention::Actual365Fixed,
            DayCountConvention::Actual360,
            BusinessDayConvention::ModifiedFollowing,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            PaymentFrequency::Quarterly,
            //
            fixing_gap_days,
            payment_gap_days,
            //
            calendar,
        )?;

        let ser = serde_json::to_string(&crs)?;
        let deser = serde_json::from_str(&ser)?;

        assert_eq!(crs.get_specific_plain_swap_type()?, PlainSwapType::CRS,);
        assert_eq!(crs.clone(), deser, "Failed to serialize and deserialize");

        let usdirs_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            Some(issue_date.clone()),
            Currency::USD,
            "USDIRS".to_string(),
            "USDIRS".to_string(),
        )?;

        let usdirs_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &usdirs_data,
            "USDIRS".to_string(),
            "USD IR Curve".to_string(),
        )?;

        let floating_curve = Rc::new(RefCell::new(usdirs_curve));

        let fixed_cashflows = crs.get_fixed_cashflows(&issue_date)?;
        let floating_cashflows =
            crs.get_floating_cashflows(&issue_date, Some(floating_curve), None)?;

        let mut fixed_keys: Vec<_> = fixed_cashflows.keys().collect();
        fixed_keys.sort();
        println!("crs fixed cashflows");
        for key in fixed_keys.iter() {
            println!("{:?}: {}", key.date(), fixed_cashflows.get(key).unwrap());
        }

        let mut floating_keys: Vec<_> = floating_cashflows.keys().collect();
        floating_keys.sort();
        println!("crs floating cashflows");
        for key in floating_keys.iter() {
            println!("{:?}: {}", key.date(), floating_cashflows.get(key).unwrap());
        }

        assert_eq!(
            fixed_cashflows.get(fixed_keys[0]).unwrap().clone(),
            1_330.0 as Real,
        );

        assert_eq!(
            fixed_cashflows.get(fixed_keys[4]).unwrap().clone(),
            -1_343.2635 as Real,
        );

        assert_eq!(
            floating_cashflows.get(floating_keys[0]).unwrap().clone(),
            -1.0 as Real,
        );

        assert_eq!(
            floating_cashflows.get(floating_keys[4]).unwrap().clone(),
            1.0100604 as Real,
        );

        Ok(())
    }
    #[test]
    fn test_fx_swap() -> Result<()> {
        let fixed_currency = Currency::KRW;
        let floating_currency = Currency::USD;
        let _unit_notional = 10_000_000.0;
        let issue_date = datetime!(2024-01-02 16:30:00 +09:00);
        let maturity = datetime!(2025-01-02 16:30:00 +09:00);
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let us = Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Settlement));
        let calendar = JointCalendar::new(vec![sk, us])?;

        let fixing_gap_days = 0;
        let payment_gap_days = 0;

        let first_fx_rate = 1300.0;
        let last_fx_rate = 1280.0;

        let initial_fixed_side_endorsement = Some(first_fx_rate);
        let initial_floating_side_payment = Some(1.0);
        let last_fixed_side_payment = Some(last_fx_rate);
        let last_floating_side_endorsement = Some(1.0);

        let instid = crate::InstId::new(
            crate::Symbol::Ticker(crate::Ticker::new(b"PlainSwap:XXX")?),
            crate::Venue::KRX,
        );
        let inst_info = crate::InstInfo::new(
            instid,
            "MockFxSwap".to_string(),
            crate::InstType::PlainSwap,
            Currency::KRW,
            1.0,
            Some(issue_date.clone()),
            Some(maturity.clone()),
            crate::AccountingLevel::L2,
        );
        let fx_swap = PlainSwap::new(
            inst_info.clone(),
            Schedule::default(),
            Schedule::default(),
            None,
            None,
            None,
            calendar,
            issue_date.clone(),
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
        )?;

        let fixed_cashflows = fx_swap.get_fixed_cashflows(&issue_date)?;
        let floating_cashflows = fx_swap.get_floating_cashflows(&issue_date, None, None)?;
        assert_eq!(
            fx_swap.get_specific_plain_swap_type()?,
            PlainSwapType::FxSwap
        );

        let mut fixed_keys: Vec<_> = fixed_cashflows.keys().collect();
        fixed_keys.sort();
        println!("fixed cashflows");
        for key in fixed_keys.iter() {
            println!("{:?}: {}", key.date(), fixed_cashflows.get(key).unwrap());
        }

        let mut floating_keys: Vec<_> = floating_cashflows.keys().collect();
        floating_keys.sort();
        println!("floating cashflows");
        for key in floating_keys.iter() {
            println!("{:?}: {}", key.date(), floating_cashflows.get(key).unwrap());
        }
        assert_eq!(
            fixed_cashflows.get(fixed_keys[0]).unwrap().clone(),
            1_300.0 as Real,
        );

        assert_eq!(
            fixed_cashflows.get(fixed_keys[1]).unwrap().clone(),
            -1_280.0 as Real,
        );

        assert_eq!(
            floating_cashflows.get(floating_keys[0]).unwrap().clone(),
            -1.0 as Real,
        );

        assert_eq!(
            floating_cashflows.get(floating_keys[1]).unwrap().clone(),
            1.0 as Real,
        );

        Ok(())
    }

    #[test]
    fn test_fx_spot() -> Result<()> {
        let fixed_currency = Currency::KRW;
        let floating_currency = Currency::USD;
        let _unit_notional = 10_000_000.0;
        let issue_date = datetime!(2024-01-02 16:30:00 +09:00);
        let maturity = datetime!(2024-01-04 16:30:00 +09:00);
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let us = Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Settlement));
        let calendar = JointCalendar::new(vec![sk, us])?;

        let fixing_gap_days = 0;
        let payment_gap_days = 0;

        let fx_rate = 1300.0;

        let initial_fixed_side_endorsement = None;
        let initial_floating_side_payment = None;
        let last_fixed_side_payment = Some(fx_rate);
        let last_floating_side_endorsement = Some(1.0);

        let instid = crate::InstId::new(
            crate::Symbol::Ticker(crate::Ticker::new(b"PlainSwap:XXX")?),
            crate::Venue::KRX,
        );

        let inst_info = crate::InstInfo::new(
            instid,
            "MockFxSpot".to_string(),
            crate::InstType::PlainSwap,
            Currency::KRW,
            1.0,
            Some(issue_date.clone()),
            Some(maturity.clone()),
            crate::AccountingLevel::L2,
        );

        let fx_spot = PlainSwap::new(
            inst_info.clone(),
            Schedule::default(),
            Schedule::default(),
            None,
            None,
            None,
            calendar,
            maturity.clone(),
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
        )?;
        let ser = serde_json::to_string(&fx_spot)?;
        let deser = serde_json::from_str(&ser)?;
        assert_eq!(fx_spot.clone(), deser, "Failed to serialize and deserialize");
        assert_eq!(
            fx_spot.get_specific_plain_swap_type()?,
            PlainSwapType::FxSpot
        );

        let fixed_cashflows = fx_spot.get_fixed_cashflows(&issue_date)?;
        let floating_cashflows = fx_spot.get_floating_cashflows(&issue_date, None, None)?;
        let mut fixed_keys: Vec<_> = fixed_cashflows.keys().collect();
        fixed_keys.sort();
        println!("fixed cashflows");
        for key in fixed_keys.iter() {
            println!("{:?}: {}", key.date(), fixed_cashflows.get(key).unwrap());
        }

        let mut floating_keys: Vec<_> = floating_cashflows.keys().collect();
        floating_keys.sort();
        println!("floating cashflows");
        for key in floating_keys.iter() {
            println!("{:?}: {}", key.date(), floating_cashflows.get(key).unwrap());
        }

        assert_eq!(
            fixed_cashflows.get(fixed_keys[0]).unwrap().clone(),
            -1_300.0 as Real,
        );

        assert_eq!(
            floating_cashflows.get(floating_keys[0]).unwrap().clone(),
            1.0 as Real,
        );
        Ok(())
    }
}
