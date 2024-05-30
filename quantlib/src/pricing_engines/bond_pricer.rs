use crate::parameters::past_price::DailyClosePrice;
use crate::instrument::InstrumentTrait;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{
    npv_result::NpvResult, 
    pricer::PricerTrait
};
use crate::instrument::Instrument;
use crate::definitions::Real;
//
use std::{
    rc::Rc, 
    cell::RefCell,
    collections::HashMap,
};
use anyhow::{Result, Context};
use time::OffsetDateTime;

/// forward_curve (Optional<Rc<RefCell<ZeroCurve>>>): forward curve for floating rate bond, so it is optional
/// past_fixing_data (Optional<Rc<CloseData>>): past fixing data for floating rate bond, so it is optional
pub struct BondPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    discount_curve: Rc<RefCell<ZeroCurve>>,
    forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
    past_fixing_data: Option<Rc<DailyClosePrice>>,
}

impl BondPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<DailyClosePrice>>,
    ) -> BondPricer {
        BondPricer {
            evaluation_date,
            discount_curve,
            forward_curve,
            past_fixing_data,
        }
    }
}

impl PricerTrait for BondPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let mut res: Real = 0.0;
        let mut disc_factor: Real;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let pricing_date = instrument.get_pricing_date()?.unwrap_or(&eval_dt);
    
        let cashflow = instrument.get_cashflows(
            &pricing_date,
            self.forward_curve.clone(),
            self.past_fixing_data.clone(),
        ).context("Failed to get coupon cashflow in calculating Bond::npv")?;

        for (payment_date, amount) in cashflow.iter() {
            if payment_date.date() > pricing_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(payment_date)?;
                res += amount * disc_factor;    
            }
        }

        res /= self.discount_curve.borrow().get_discount_factor_at_date(&pricing_date)?;
        Ok(res)

    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let pricing_date = instrument.get_pricing_date()?.unwrap_or(&eval_dt);

        let mut npv: Real = 0.0;
        let mut coupon_amounts: HashMap<usize, (OffsetDateTime, Real)> = HashMap::new();
        let mut coupon_payment_probability: HashMap<usize, (OffsetDateTime, Real)> = HashMap::new();

        let mut disc_factor: Real;
        
        let cashflow = instrument.get_cashflows(
            &pricing_date,
            self.forward_curve.clone(),
            self.past_fixing_data.clone(),
        ).context("Failed to get coupon cashflow in calculating Bond::npv_result")?; // include evaluation date

        for (i, (payment_date, amount)) in cashflow.iter().enumerate() {
            if pricing_date.date() < payment_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(payment_date)?;
                npv += amount * disc_factor;    
            }

            if pricing_date.date() <= payment_date.date () {
                coupon_amounts.insert(i as usize, (payment_date.clone(), *amount));
                coupon_payment_probability.insert(i, (payment_date.clone(), 1.0));
            }
        }

        npv /= self.discount_curve.borrow().get_discount_factor_at_date(&pricing_date)?;

        let res = NpvResult::new(
            npv,
            coupon_amounts,
            coupon_payment_probability,
        );

        Ok(res)
    }       
}

// please make a pricer test by refering crate::instruments::schedule, 
// crate::parameters::zero_curve::ZeroCurve, crate::evaluation_date::EvaluationDate,
// crate::pricing_engines::{npv_result::NpvResult, pricer::PricerTrait},
// crate::instrument::Instrument, crate::instruments::bond::fixed_coupon_bond
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::evaluation_date::EvaluationDate;
    use crate::pricing_engines::pricer::PricerTrait;
    use crate::instrument::Instrument;
    use crate::definitions::Real;
    use crate::data::vector_data::VectorData;
    use crate::time::conventions::{DayCountConvention, BusinessDayConvention, PaymentFrequency};
    use crate::instruments::bond::Bond;
    use crate::currency::Currency;
    use crate::enums::{CreditRating, IssuerType, RankType};
    use crate::time::{
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        calendar::Calendar,
        jointcalendar::JointCalendar,
    };
    use crate::parameters::rate_index::RateIndex;
    //use crate::enums::RateIndexCode;
    //
    use std::{rc::Rc, cell::RefCell};
    use time::{Duration, macros::datetime};
    use ndarray::array;
    use anyhow::Result;

    #[test]
    fn test_fixed_coupon_bond_pricer() -> Result<()> {
        let dt = datetime!(2021-01-01 16:30:00 +09:00);
        let bond_pricing_date = dt.clone();
        let name = "KRWGOV";
        let evaluation_date = Rc::new(RefCell::new(
            EvaluationDate::new(dt),
        ));

        // define a vector data 1Y = 0.03, 5Y = 0.04
        let curve_data = VectorData::new(
            array!(0.03, 0.03),
            None,
            Some(array!(1.0, 5.0)),
            None,//evaluation_date.borrow().get_date_clone(),
            Currency::KRW,
            name.to_string(),
        )?;

        // make a discount curve (ZeroCurve)
        let discount_curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &curve_data,
                name.to_string(),
                name.to_string(),
            )?
        ));
        
        // make a pricer
        let pricer = BondPricer::new(
            evaluation_date.clone(),
            discount_curve.clone(),
            None,
            None,
        );

        // let's make a fixed coupnon bond paying quaterly 3% coupon
        let issuedate = datetime!(2020-01-01 16:30:00 +09:00);
        let maturity = issuedate + Duration::days(365 * 4);
        let issuer_name = "Korea Government";
        let bond_name = "KRW Fixed Coupon Bond";
        let bond_code = "KR1234567890";
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        
        let calendar = JointCalendar::new(vec![sk])?;

        let bond = Bond::new_from_conventions(
            IssuerType::Government,
            CreditRating::None, 
            issuer_name.to_string(),
            RankType::Senior, 
            Currency::KRW,
            //
            10_000.0, 
            false, 
            //
            issuedate.clone(),
            issuedate.clone(),
            None,
            maturity,
            //
            Some(0.03), 
            None,
            None,
            None,
            //
            calendar,
            //
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::Quarterly,
            //
            0,
            0,             
            bond_name.to_string(), 
            bond_code.to_string(),
        )?;

        let cashflows = bond.get_cashflows(
            &bond_pricing_date,
            None,
            None,
        )?;


        let filtered_cashflows: HashMap<OffsetDateTime, Real> = cashflows
            .iter()
            .filter(|(&key, _)| key > dt)
            .map(|(&key, &value)| (key, value))
            .collect();

        let mut keys: Vec<_> = filtered_cashflows.keys().collect();
        keys.sort();
        for key in keys.iter() {
            println!("{:?}: {}", key.date(), filtered_cashflows.get(key).unwrap());
        }
        
        let cashflow_sum = filtered_cashflows.iter().fold(0.0, |acc, (_, amount)| acc + amount);
        
        let expected_sum = 1.09;
        assert!(
            (cashflow_sum - expected_sum).abs() < 1.0e-5,
            "{}:{}  cashflow_sum: {}, expected: {} (did you change the pricer or definition of Real?)",
            file!(),
            line!(),
            cashflow_sum,
            expected_sum
        );

        let isntrument = Instrument::Bond(bond.clone());
        let expected_npv: Real = 0.99976;

        let npv_result = pricer.npv_result(&isntrument)?;
        let npv = npv_result.get_npv();

        assert!(
            (npv - expected_npv).abs() < 1.0e-5,
            "{}:{}  npv_result.get_npv(): {}, expected: {} (did you change the pricer or definition of Real?)",
            file!(),
            line!(),
            npv,
            expected_npv
        );

        Ok(())
    }

    #[test]
    // test bond pricer for floating rate note
    // which calculate overnight rate (compound_tenor = String::from("1D")) + spread
    fn test_floating_rate_note_pricer() -> Result<()> {
        let dt = datetime!(2020-12-31 16:30:00 +09:00);
        let effective_date = datetime!(2021-01-01 16:30:00 +09:00);
        let name = "KRWGOV";
        let evaluation_date = Rc::new(RefCell::new(
            EvaluationDate::new(dt),
        ));

        // define a vector data 1Y = 0.03, 5Y = 0.04
        let curve_data = VectorData::new(
            array!(0.04, 0.04),
            None,
            Some(array!(1.0, 5.0)),
            None,//evaluation_date.borrow().get_date_clone(),
            Currency::KRW,
            name.to_string(),
        )?;

        // make a discount curve (ZeroCurve)
        let discount_curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &curve_data,
                name.to_string(),
                name.to_string(),
            )?
        ));

        // define a vector data 1Y = 0.03, 5Y = 0.04
        let forward_curve_data = VectorData::new(
            array!(0.04, 0.04),
            None,
            Some(array!(1.0, 5.0)),
            None,//evaluation_date.borrow().get_date_clone(),
            Currency::KRW,
            name.to_string(),
        )?;

        // make a discount curve (ZeroCurve)
        let forward_curve = Rc::new(RefCell::new(
            ZeroCurve::new(
                evaluation_date.clone(),
                &forward_curve_data,
                "KRWIRS".to_string(),
                "KRWIRS".to_string(),
            )?
        ));
        
        // make a pricer
        let pricer = BondPricer::new(
            evaluation_date.clone(),
            discount_curve.clone(),
            Some(forward_curve.clone()),
            None,
        );

        // let's make a floating rate note paying quaterly 3% coupon
        let issuedate = datetime!(2020-12-31 16:30:00 +09:00);
        let maturity = issuedate + Duration::days(365 * 4);
        let issuer_name = "Korea Government";
        let bond_name = "KRW Floating Rate Note";
        let bond_code = "KR1234567890";
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;
        let rate_index = RateIndex::new(
            String::from("91D"),
            Currency::KRW,
            String::from("CD 91D"),
            String::from("CD 91D"),
        )?;

        let bond = Bond::new_from_conventions(
            IssuerType::Government,
            CreditRating::None, 
            issuer_name.to_string(),
            RankType::Senior, 
            Currency::KRW,
            //
            10_000.0, 
            false, 
            //
            issuedate.clone(),
            effective_date,
            None,
            maturity,
            //
            None,
            Some(0.00),
            Some(rate_index),
            None,//Some(String::from("1D")),
            //
            calendar,
            //
            
            true,
            DayCountConvention::ActActIsda,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::Quarterly,
            //
            1,
            0,
            bond_name.to_string(),
            bond_code.to_string(),
        )?;

        let cashflows = bond.get_cashflows(
            &dt,
            Some(forward_curve.clone()), 
            None)?;

        let mut keys: Vec<_> = cashflows.keys().collect();
        keys.sort();
        for key in keys.iter() {
            println!("{:?}: {}", key.date(), cashflows.get(key).unwrap());
        }
    
        let npv = pricer.npv(&Instrument::Bond(bond.clone()))?;
        let expected_npv = 0.998420;
        assert!(
            (npv - expected_npv).abs() < 1.0e-5,
            "{}:{}  npv: {}, expected: {} (did you change the pricer or definition of Real?)",
            file!(),
            line!(),
            npv,
            expected_npv
        );
        Ok(())
    }
}

