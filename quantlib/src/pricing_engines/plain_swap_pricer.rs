use crate::assets::{
    currency::Currency,
    fx::{FxCode, FX},
};
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::data::history_data::CloseData;
use crate::pricing_engines::{
    pricer::PricerTrait,
    npv_result::NpvResult,
};
use crate::instrument::{
    Instrument,
    InstrumentTrait,
};
use crate::definitions::Real;
// 
use std::{
    cell::RefCell,
    rc::Rc,
    collections::HashMap,
};
use time::OffsetDateTime;
use anyhow::{Context, Result, anyhow};

pub struct PlainSwapPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    fixed_leg_discount_curve: Rc<RefCell<ZeroCurve>>,
    floating_leg_discount_curve: Rc<RefCell<ZeroCurve>>,
    forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
    past_fixing_data: Option<Rc<CloseData>>,
    fxs: Option<HashMap<FxCode, Rc<RefCell<FX>>>>,
}

impl PlainSwapPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fixed_leg_discount_curve: Rc<RefCell<ZeroCurve>>,
        floating_leg_discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<CloseData>>,
        fxs: Option<HashMap<FxCode, Rc<RefCell<FX>>>>,
    ) -> Result<PlainSwapPricer> {
        Ok(PlainSwapPricer {
            evaluation_date,
            fixed_leg_discount_curve,
            floating_leg_discount_curve,
            forward_curve,
            past_fixing_data,
            fxs,
        })
    }
}

impl PricerTrait for PlainSwapPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let fixed_leg_fx_rate = match self.fxs {
            Some(ref fxs) => {
                let fixed_fx_code = FxCode::new(
                    instrument.get_fixed_leg_currency()?.clone(), 
                    Currency::KRW,
                );

                fxs.get(&fixed_fx_code)
                    .with_context(|| anyhow!(
                        "({}:{}) The pricer of does not have fx rate of {:?} for {} ({})", 
                        file!(), line!(), fixed_fx_code, instrument.get_name(), instrument.get_code(),
                    ))?
                    .borrow()
                    .get_rate()
            },
            None => 1.0,
        };
        
        let floating_leg_fx_rate = match self.fxs {
            Some(ref fxs) => {
                let floating_fx_code = FxCode::new(
                    instrument.get_floating_leg_currency()?.clone(), 
                    Currency::KRW,
                );

                fxs.get(&floating_fx_code)
                    .with_context(|| anyhow!(
                        "({}:{}) The pricer of does not have fx rate of {:?} for {} ({})", 
                        file!(), line!(), floating_fx_code, instrument.get_name(), instrument.get_code(),
                    ))?
                    .borrow()
                    .get_rate()
            },
            None => 1.0,
        };

        let mut cashflow_amounts: HashMap<usize, (OffsetDateTime, Real)> = HashMap::new();
        let mut cashflow_probabilities: HashMap<usize, (OffsetDateTime, Real)> = HashMap::new();
        let mut fixed_res = 0.0;
        let mut floating_res = 0.0;
        let mut discount_factor: Real;
        let eval_date = self.evaluation_date.borrow().get_date_clone();
        let fixed_cashflows = instrument.get_fixed_cashflows(&eval_date)?;
        let floating_cashflows = instrument.get_floating_cashflows(
            &eval_date,
            self.forward_curve.clone(),
            self.past_fixing_data.clone(),
        )?;

        let fixed_leg_discount_curve = self.fixed_leg_discount_curve.borrow();
        let floating_leg_discount_curve = self.floating_leg_discount_curve.borrow();

        let mut count: usize = 0;
        for (payment_date, amount) in fixed_cashflows.iter() {
            if eval_date.date() < payment_date.date() {
                discount_factor = fixed_leg_discount_curve.get_discount_factor_at_date(payment_date)?;
                fixed_res += amount * discount_factor;    
            }
            if eval_date.date() <= payment_date.date() {
                cashflow_amounts.insert(count, (payment_date.clone(), amount.clone()));
                cashflow_probabilities.insert(count, (payment_date.clone(), 1.0));
                count += 1;
            }     
        }

        for (payment_date, amount) in floating_cashflows.iter() {
            if eval_date.date() < payment_date.date() {
                discount_factor = floating_leg_discount_curve.get_discount_factor_at_date(payment_date)?;
                floating_res += amount * discount_factor;
            }
            if eval_date.date() <= payment_date.date() {
                cashflow_amounts.insert(count, (payment_date.clone(), amount.clone()));
                cashflow_probabilities.insert(count, (payment_date.clone(), 1.0));
                count += 1;
            }
        }

        let res = fixed_res * fixed_leg_fx_rate - floating_res * floating_leg_fx_rate;

        let npv_result = NpvResult::new(
            res,
            cashflow_amounts,
            cashflow_probabilities,
        );

        Ok(npv_result)
       
    }

    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let fixed_leg_fx_rate = match self.fxs {
            Some(ref fxs) => {
                let fixed_fx_code = FxCode::new(
                    instrument.get_fixed_leg_currency()?.clone(),
                    Currency::KRW,
                );

                fxs.get(&fixed_fx_code)
                    .with_context(|| anyhow!(
                        "({}:{}) The pricer of does not have fx rate of {:?} for {} ({})", 
                        file!(), line!(), fixed_fx_code, instrument.get_name(), instrument.get_code(),
                    ))?
                    .borrow()
                    .get_rate()
            },
            None => 1.0,
        };
        
        let floating_leg_fx_rate = match self.fxs {
            Some(ref fxs) => {
                let floating_fx_code = FxCode::new(
                    instrument.get_floating_leg_currency()?.clone(), 
                    Currency::KRW,
                );

                fxs.get(&floating_fx_code)
                    .with_context(|| anyhow!(
                        "({}:{}) The pricer of does not have fx rate of {:?} for {} ({})", 
                        file!(), line!(), floating_fx_code, instrument.get_name(), instrument.get_code(),
                    ))?
                    .borrow()
                    .get_rate()
            },
            None => 1.0,
        };

        let mut fixed_res = 0.0;
        let mut floating_res = 0.0;
        let mut discount_factor: Real;
        let eval_date = self.evaluation_date.borrow().get_date_clone();
        let fixed_cashflows = instrument.get_fixed_cashflows(&eval_date)?;
        let floating_cashflows = instrument.get_floating_cashflows(
            &eval_date,
            self.forward_curve.clone(),
            self.past_fixing_data.clone(),
        )?;

        let fixed_leg_discount_curve = self.fixed_leg_discount_curve.borrow();
        let floating_leg_discount_curve = self.floating_leg_discount_curve.borrow();

        for (payment_date, amount) in fixed_cashflows.iter() {
            if eval_date.date() < payment_date.date() {
                discount_factor = fixed_leg_discount_curve.get_discount_factor_at_date(payment_date)?;
                fixed_res += amount * discount_factor;    
            }
            
        }
        for (payment_date, amount) in floating_cashflows.iter() {
            if eval_date.date() < payment_date.date() {
                discount_factor = floating_leg_discount_curve.get_discount_factor_at_date(payment_date)?;
                floating_res += amount * discount_factor;
            }
        }

        let res = fixed_res * fixed_leg_fx_rate - floating_res * floating_leg_fx_rate;
        Ok(res)
    }

    fn fx_exposure(&self,instrument: &Instrument, _npv: Real) -> Result<HashMap<Currency, Real>> {
        let mut fixed_res = 0.0;
        let mut floating_res = 0.0;
        let mut discount_factor: Real;
        let eval_date = self.evaluation_date.borrow().get_date_clone();
        let fixed_cashflows = instrument.get_fixed_cashflows(&eval_date)?;
        let floating_cashflows = instrument.get_floating_cashflows(
            &eval_date,
            self.forward_curve.clone(),
            self.past_fixing_data.clone(),
        )?;

        let fixed_leg_discount_curve = self.fixed_leg_discount_curve.borrow();
        let floating_leg_discount_curve = self.floating_leg_discount_curve.borrow();

        for (payment_date, amount) in fixed_cashflows.iter() {
            if eval_date.date() < payment_date.date() {
                discount_factor = fixed_leg_discount_curve.get_discount_factor_at_date(payment_date)?;
                fixed_res += amount * discount_factor;    
            }
        }

        for (payment_date, amount) in floating_cashflows.iter() {
            if eval_date.date() < payment_date.date() {
                discount_factor = floating_leg_discount_curve.get_discount_factor_at_date(payment_date)?;
                floating_res += amount * discount_factor;
            }
        }

        let mut res: HashMap<Currency, Real> = HashMap::new();
        res.insert(instrument.get_fixed_leg_currency()?.clone(), fixed_res * instrument.get_unit_notional());
        res.insert(instrument.get_floating_leg_currency()?.clone(), - floating_res * instrument.get_unit_notional());

        Ok(res)
        
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::assets::{
        currency::Currency,
        fx::{FxCode, FX},
    };
    use crate::enums::RateIndexCode;
    use crate::parameters::{
        zero_curve::ZeroCurve,
        rate_index::RateIndex
    };
    use crate::data::vector_data::VectorData;
    use crate::evaluation_date::EvaluationDate;
    use crate::instrument::{
        Instrument,
        InstrumentTrait,
    };
    use crate::instruments::plain_swap::{
        PlainSwap,
        PlainSwapType,
    };

    use crate::definitions::Real;
    use crate::time::{
        calendar_trait::CalendarTrait,
        calendar::Calendar,
        jointcalendar::JointCalendar,
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        calendars::unitedstates::{UnitedStates, UnitedStatesType},
        conventions::{
            DayCountConvention,
            BusinessDayConvention,
            PaymentFrequency,
        },
    };
    use std::{
        cell::RefCell,
        rc::Rc,
        collections::HashMap,
    };
    use time::{
        OffsetDateTime,
        Duration,
        macros::datetime,
    };
    use anyhow::Result;
    use ndarray::array;

    #[test]
    fn test_crs_pricer() -> Result<()> {
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
            RateIndexCode::LIBOR,
            String::from("USD Libor 3M") // this is just a mock code
        )?;

        let initial_fixed_side_endorsement = Some(fx_rate);
        let initial_floating_side_payment = Some(1.0);
        let last_fixed_side_payment = Some(fx_rate);
        let last_floating_side_endorsement = Some(1.0);
        
        let crs = PlainSwap::new_from_conventions(
            fixed_currency,
            floating_currency,
            //
            initial_fixed_side_endorsement,
            initial_floating_side_payment,
            last_fixed_side_payment,
            last_floating_side_endorsement,
            //
            unit_notional,
            issue_date.clone(),
            effective_date.clone(),
            maturity.clone(),
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
            "MockCRS".to_string(),
            "MockCode".to_string(),
        )?;

        assert_eq!(
            crs.get_specific_type(),
            PlainSwapType::CRS,
        );

        let usdirs_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            issue_date.clone(),
            Currency::USD,
            "USDIRS".to_string(),
        )?;
        
        let usdirs_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &usdirs_data,
            "USDIRS".to_string(),
            "USD IR Curve".to_string(),
        )?;

        let floating_curve = Rc::new(RefCell::new(usdirs_curve));

        let mut fxs = HashMap::new();
        let fx_code = FxCode::new(Currency::KRW, Currency::KRW);
        fxs.insert(
            &fx_code,
            Rc::new(RefCell::new(FX::new(1.0, fx_code, datetime!(2024-01-02 16:30:00 +09:00)))),
        );
        let fx_code = FxCode::new(Currency::USD, Currency::KRW);
        fxs.insert(
            &fx_code,
            Rc::new(RefCell::new(FX::new(fx_rate, fx_code, datetime!(2024-01-02 16:30:00 +09:00)))),
        );
        
        Ok(())
    }
}