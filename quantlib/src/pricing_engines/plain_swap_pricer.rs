use crate::currency::{Currency, FxCode};
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
use crate::market_price::MarketPrice;
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
    floating_to_fixed_fx: Option<Rc<RefCell<MarketPrice>>>,
}

impl PlainSwapPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        fixed_leg_discount_curve: Rc<RefCell<ZeroCurve>>,
        floating_leg_discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<CloseData>>,
        floating_to_fixed_fx: Option<Rc<RefCell<MarketPrice>>>,
    ) -> Result<PlainSwapPricer> {
        Ok(PlainSwapPricer {
            evaluation_date,
            fixed_leg_discount_curve,
            floating_leg_discount_curve,
            forward_curve,
            past_fixing_data,
            floating_to_fixed_fx,
        })
    }
}

impl PricerTrait for PlainSwapPricer {
    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let floating_to_fixed_fx = match self.floating_to_fixed_fx {
            Some(ref fxf) => fxf.borrow().get_value(),
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
                cashflow_amounts.insert(count, (payment_date.clone(), amount * floating_to_fixed_fx));
                cashflow_probabilities.insert(count, (payment_date.clone(), 1.0));
                count += 1;
            }
        }

        let res = fixed_res + floating_res * floating_to_fixed_fx;

        let npv_result = NpvResult::new(
            res,
            cashflow_amounts,
            cashflow_probabilities,
        );

        Ok(npv_result)
    
    }

    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let floating_to_fixed_fx_rate = match self.floating_to_fixed_fx {
            Some(ref fxf) => fxf.borrow().get_value(),
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

        let res = fixed_res  + floating_res * floating_to_fixed_fx_rate;
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

        let fixed_currency = instrument.get_fixed_leg_currency()?;
        let floating_currency = instrument.get_floating_leg_currency()?;
        let fixed_amount = fixed_res * instrument.get_unit_notional();
        let floating_amount = floating_res * instrument.get_unit_notional();
        let mut res: HashMap<Currency, Real> = HashMap::new();

        res.entry(fixed_currency.clone())
            .and_modify(|v| *v += fixed_amount)
            .or_insert(fixed_amount);

        res.entry(floating_currency.clone())
            .and_modify(|v| *v += floating_amount)
            .or_insert(floating_amount);

        Ok(res)
        
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::currency::{Currency, FxCode};
    use crate::pricing_engines::pricer::{PricerTrait, Pricer};
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
        let unit_notional = 1.0;
        let issue_date = datetime!(2024-01-02 16:30:00 +09:00);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(
            issue_date + Duration::days(4),
        )));
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

        let inst = Instrument::PlainSwap(crs);

        assert_eq!(
            inst.get_specific_plain_swap_type()?,
            PlainSwapType::CRS,
        );

        let usdirs_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            None,//issue_date.clone(),
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

        let krwcrs_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            None,
            Currency::KRW,
            "KRWCRS".to_string(),
        )?;
        
        let krwcrs_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &krwcrs_data,
            "KRWCRS".to_string(),
            "KRW CRS Curve".to_string(),
        )?;

        let fixed_curve = Rc::new(RefCell::new(krwcrs_curve));

        
        let fx_code = FxCode::new(Currency::USD, Currency::KRW);
        
        let floating_to_fixed_fx = Rc::new(RefCell::new(
            MarketPrice::new(
                fx_rate,
                datetime!(2024-01-02 16:30:00 +09:00),
                None,
                fx_code.get_currency2().clone(),
                fx_code.to_string(),
                fx_code.to_string(),
            )
        ));
        

        let pricer = PlainSwapPricer::new(
            evaluation_date.clone(),
            fixed_curve.clone(),
            floating_curve.clone(),
            Some(floating_curve.clone()),
            None,
            Some(floating_to_fixed_fx.clone()),
        )?;


        let npv_result = pricer.npv_result(&inst)?;
        let npv = pricer.npv(&inst)?;
        let fx_exposure = pricer.fx_exposure(
            &inst, 
            npv_result.get_npv(),
        )?;

        println!("NPV: {:?}", npv);
        println!("Cashflows:");
        for i in 0..npv_result.get_cashflow_amounts().len() {
            let (date, amount) = npv_result.get_cashflow_amounts().get(&i).unwrap();
            println!("{:?}: {}", date.date(), amount);
        }
        println!("FX Exposure: {:?}", fx_exposure);
        
        //NPV: 0.460083
        //FX Exposure: {KRW: -13_303_186_000.0, USD: 10_005_854.0}
        let expected_npv = 0.460083;
        let expected_krw_exposure = -1_330.3186;
        let expected_usd_exposure = 1.0005854;

        assert!(
            (npv - npv_result.get_npv()).abs() / fx_rate < 1e-5,
            "npv: {}, npv_result.get_npv(): {}", npv, npv_result.get_npv(),
        );

        assert!(
            (npv - expected_npv).abs() / fx_rate < 1e-5,
            "npv: {}, expected_npv: {}", npv, expected_npv,
        );

        assert!(
            (fx_exposure.get(&Currency::KRW).unwrap() - expected_krw_exposure).abs() < 1e-6,
            "fx_exposure: {}, expected_krw_exposure: {}", fx_exposure.get(&Currency::KRW).unwrap(), expected_krw_exposure,
        );
        
        assert!(
            (fx_exposure.get(&Currency::USD).unwrap() - expected_usd_exposure).abs() < 1e-6,
            "fx_exposure: {}, expected_usd_exposure: {}", fx_exposure.get(&Currency::USD).unwrap(), expected_usd_exposure,
        );

        Ok(())
    }

    #[test]
    fn test_irs_pricer() -> Result<()> {
        let fixed_currency = Currency::KRW;
        let floating_currency = Currency::KRW;
        let unit_notional = 100.0;
        let issue_date = datetime!(2024-01-02 16:30:00 +09:00);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(
            issue_date + Duration::days(4),
        )));
        let effective_date = datetime!(2024-01-03 16:30:00 +09:00);
        let maturity = datetime!(2025-01-03 16:30:00 +09:00);
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        // let us = Calendar::UnitedStates(UnitedStates::new(UnitedStatesType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;
        
        let fixing_gap_days = 1;
        let payment_gap_days = 0;

        let fixed_rate = 0.04;
        let rate_index = RateIndex::new(
            String::from("91D"),
            Currency::KRW,
            RateIndexCode::CD,
            String::from("CD91") // this is just a mock code
        )?;

        let initial_fixed_side_endorsement = None;
        let initial_floating_side_payment = None;
        let last_fixed_side_payment = None;
        let last_floating_side_endorsement = None;
        
        let irs = PlainSwap::new_from_conventions(
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
            false,
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
            "MockIRS".to_string(),
            "MockCode".to_string(),
        )?;

        let curve_data = VectorData::new(
            array![0.04, 0.04],
            None,
            Some(array![0.5, 5.0]),
            None,
            Currency::KRW,
            "KRWIRS".to_string(),
        )?;
        
        let curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &curve_data,
                "KRWIRS".to_string(),
                "KRW IR Curve".to_string(),
            )?
        ));

        let pricer = PlainSwapPricer::new(
            evaluation_date.clone(),
            curve.clone(),
            curve.clone(),
            Some(curve.clone()),
            None,
            None,
        )?;

        let inst = Instrument::PlainSwap(irs);
        let npv_result = pricer.npv_result(&inst)?;
        let fx_exposure = pricer.fx_exposure(
            &inst, 
            npv_result.get_npv(),
        )?;

        let npv_from_npv_result = npv_result.get_npv();
        let npv = pricer.npv(&inst)?;
        let expected_npv = 0.0003459379;
        let expected_fx_exposure = 0.03459382;

        println!("NPV: {:?}", npv);
        println!("Cashflows:");
        for i in 0..npv_result.get_cashflow_amounts().len() {
            let (date, amount) = npv_result.get_cashflow_amounts().get(&i).unwrap();
            println!("{:?}: {}", date.date(), amount);
        }
        println!("FX Exposure: {:?}", fx_exposure);

        assert!(
            (npv - expected_npv).abs() < 1e-6,
            "npv: {}, expected_npv: {}", npv, expected_npv,
        );

        assert!(
            (npv - npv_from_npv_result).abs() < 1e-6,
            "npv: {}, npv_from_npv_result: {}", npv, npv_from_npv_result,
        );

        assert!(
            (fx_exposure.get(&Currency::KRW).unwrap() - expected_fx_exposure).abs() < 1e-6,
            "fx_exposure: {}, expected_fx_exposure: {}", fx_exposure.get(&Currency::KRW).unwrap(), expected_fx_exposure,
        );

        Ok(())
    }
}