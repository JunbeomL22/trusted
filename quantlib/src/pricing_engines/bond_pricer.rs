use crate::instrument::InstrumentTriat;
use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{npv_result::NpvResult, pricer::PricerTrait};
use crate::instrument::Instrument;
use crate::definitions::Real;
//
use anyhow::{Context, Result};
use std::{
    rc::Rc, 
    cell::RefCell,
    collections::HashMap,
};
use time::OffsetDateTime;

pub struct BondPricer{
    discount_curve: Rc<RefCell<ZeroCurve>>,
    forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
}

impl BondPricer {
    pub fn new(
        discount_curve: Rc<RefCell<ZeroCurve>>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>
    ) -> BondPricer {
        BondPricer {
            discount_curve,
            forward_curve,
            evaluation_date,
        }
    }
}

impl PricerTrait for BondPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let mut res: Real = 0.0;
        let mut disc_factor: Real;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let pricing_date = instrument.get_pricing_date()?.unwrap_or(&eval_dt);
    
        let cashflow = instrument.get_coupon_cashflow(
            &pricing_date,
            self.forward_curve.clone(),
        ).context("Failed to get coupon cashflow in calculating FixedCouponBond::npv")?;

        for (payment_date, amount) in cashflow.iter() {
            if payment_date.date() > pricing_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(payment_date)?;
                res += amount * disc_factor;    
            }
        }

        if !instrument.is_coupon_strip()? {
            let maturity = instrument.get_maturity().unwrap();

            if maturity.date() > pricing_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(maturity)?;
                res += disc_factor;
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
        
        let cashflow = instrument.get_coupon_cashflow(
            &pricing_date,
            self.forward_curve.clone(),
        ).context("Failed to get coupon cashflow in calculating FixedCouponBond::npv_result")?; // include evaluation date

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

        if !instrument.is_coupon_strip()? {
            let maturity = instrument.get_maturity().unwrap();
            if maturity.date() > pricing_date.date() {
                disc_factor = self.discount_curve.borrow().get_discount_factor_at_date(maturity)?;
                npv += disc_factor;
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
    use std::{rc::Rc, cell::RefCell};
    use time::{Duration, macros::datetime};
    use crate::data::vector_data::VectorData;
    use ndarray::array;
    use crate::time::conventions::{DayCountConvention, BusinessDayConvention, PaymentFrequency};
    use crate::instruments::bonds::fixed_coupon_bond::FixedCouponBond;
    use crate::assets::currency::Currency;
    use anyhow::Result;
    use crate::enums::{CreditRating, IssuerType, RankType};
    use crate::time::{
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        calendar::Calendar,
        jointcalendar::JointCalendar,
    };

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
            evaluation_date.borrow().get_date_clone(),
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
            discount_curve.clone(),
            None,
            evaluation_date.clone(),
        );

        // let's make a fixed coupnon bond paying quaterly 3% coupon
        let issuedate = datetime!(2020-01-01 16:30:00 +09:00);
        let maturity = issuedate + Duration::days(365 * 4);
        let issuer_name = "Korea Government";
        let bond_name = "KRW Fixed Coupon Bond";
        let bond_code = "KR1234567890";
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        
        let calendar = JointCalendar::new(vec![sk])?;

        let bond = FixedCouponBond::new_from_conventions(
            Currency::KRW,
            IssuerType::Government, 
            CreditRating::None, 
            RankType::Senior, 
            false, 
            0.03, 
            10_000.0, 
            issuedate.clone(), 
            issuedate.clone(),
            maturity,
            None, 
            Some(bond_pricing_date.clone()),
            calendar,
            DayCountConvention::ActActIsda, 
            BusinessDayConvention::Unadjusted, 
            PaymentFrequency::SemiAnnually, 
            issuer_name.to_string(), 
            0,             
            bond_name.to_string(), 
            bond_code.to_string(),
        )?;

        let cashflows = bond.get_coupon_cashflow(
            &bond_pricing_date,
            None,
        )?;


        let filtered_cashflows: HashMap<OffsetDateTime, Real> = cashflows
            .iter()
            .filter(|(&key, _)| key > dt)
            .map(|(&key, &value)| (key, value))
            .collect();

        let cashflow_sum = filtered_cashflows.iter().fold(0.0, |acc, (_, amount)| acc + amount);
        
        let expected_sum = 0.08991781;
        assert!(
            (cashflow_sum - expected_sum).abs() < 1.0e-5,
            "{}:{}  cashflow_sum: {}, expected: {} (did you change the pricer or definition of Real?)",
            file!(),
            line!(),
            cashflow_sum,
            expected_sum
        );

        let isntrument = Instrument::FixedCouponBond(bond.clone());
        let expected_npv: Real = 0.9993578;

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
}

