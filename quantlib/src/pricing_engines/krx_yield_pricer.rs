use crate::definitions::Real;
use crate::instruments::fixed_coupon_bond::FixedCouponBond;
use crate::pricing_engines::pricer::PricerTrait;
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::{Instrument, InstrumentTriat};
use crate::evaluation_date::EvaluationDate;
use crate::time::{
    conventions::DayCountConvention,
    calendars::calendar_trait::CalendarTrait,
};
use crate::parameters::zero_curve::ZeroCurve;
//
use anyhow::{Result, Context, anyhow};
use std::{
    rc::Rc,
    cell::RefCell,
};
use argmin::core::{Error, Executor, CostFunction, Gradient};
use argmin::solver::gradientdescent::SteepestDescent;
use argmin::solver::linesearch::MoreThuenteLineSearch;

/// 금융투자회사의 영업 및 업무에 관한 규정 별표 14
/// https://law.kofia.or.kr/service/law/lawFullScreenContent.do?seq=136&historySeq=263
#[derive(Debug, Clone)]
pub struct KrxYieldPricer {
    bond_yield: Real,
    forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    daycount: DayCountConvention,
}

impl KrxYieldPricer {
    pub fn new(
        bond_yield: Real, 
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> KrxYieldPricer {
        KrxYieldPricer { 
            bond_yield,
            forward_curve,
            evaluation_date,
            daycount: DayCountConvention::StreetConvention,
        }
    }

    pub fn get_bond_yield(&self) -> Real {
        self.bond_yield
    }

    pub fn set_bond_yield(&mut self, bond_yield: Real) {
        self.bond_yield = bond_yield;
    }

    pub fn find_bond_yield(
        &self, 
        bond: FixedCouponBond, 
        npv: Real,
    ) -> Result<Real> {
        let pricer = self.clone();
        let problem = KrxYieldPricerCostFunction::new(bond, npv, pricer);
        //let solver = LBFGS::new(MoreThuenteLineSearch::new(), 3).with_tolerance_cost(1e-5)?;
        //let solver = BFGS::new(MoreThuenteLineSearch::new()).with_tolerance_cost(1e-5)?;
        // Set up line search needed by `SteepestDescent`
        let linesearch = MoreThuenteLineSearch::new();        
        // Set up solver -- `SteepestDescent` requires a linesearch
        let solver = SteepestDescent::new(linesearch);
        let init_param = 0.02;
        let executor = Executor::new(problem, solver)
            .configure(|state|
                state
                    .param(init_param)
                    .max_iters(30)
                    .target_cost(1.0e-12)
            );
        
        let res = executor.run()?;
        //println!("res: {:?}", res.state);
        match res.state.best_param {
            Some(param) => Ok(param),
            None => Err(anyhow!("Failed to find bond yield")),
        
        }
    }
}

pub struct KrxYieldPricerCostFunction {
    bond: Instrument,
    npv: Real,
    pricer: RefCell<KrxYieldPricer>,
}

impl KrxYieldPricerCostFunction {
    pub fn new(bond: FixedCouponBond, npv: Real, pricer: KrxYieldPricer) -> KrxYieldPricerCostFunction {
        KrxYieldPricerCostFunction {
            bond: Instrument::FixedCouponBond(bond),
            npv,
            pricer: RefCell::new(pricer),
        }
    }
}

impl CostFunction for KrxYieldPricerCostFunction {
    type Param = Real;
    type Output = Real;
    

    fn cost(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        {self.pricer.borrow_mut().set_bond_yield(*param);}
        let npv = self.pricer.borrow().npv(&self.bond)?;
        Ok((npv - self.npv).powf(2.0))
    }
}

impl Gradient for KrxYieldPricerCostFunction {
    type Param = Real;
    type Gradient = Real;
  
    fn gradient(&self, param: &Self::Param) -> Result<Self::Gradient, Error> {
        let h = 1e-7;
        let grad = (self.cost(&(param + h))? - self.cost(&(param - h))?)/(2.0*h);
        Ok(grad)
    }
}

impl PricerTrait for KrxYieldPricer {
    fn npv(&self, bond: &Instrument) -> Result<Real> {
        let mut res: Real = 0.0;
        let mut disc_factor: Real;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let pricing_date = bond.get_pricing_date()?.unwrap_or(&eval_dt);
        let freq = bond.get_frequency()?.as_real();
        let effective_yield = self.bond_yield / freq;
        let cal = bond.get_calendar()?;
        let mut diff: Real;
        let daycounter = self.daycount;
        let maturity = bond.get_maturity().unwrap();

        let cashflow = bond.get_coupon_cashflow(
            &pricing_date,
            self.forward_curve.clone(),
        ).context("Failed to get coupon cashflow in calculating FixedCouponBond::npv")?;

        // get minimum date after the pricing date (the key of cashflow)
        let min_cashflow_date = cashflow
            .keys()
            .filter(|&date| date.date() > pricing_date.date())
            .min()
            .unwrap_or(&maturity);
        
        if cashflow.len() > 0 {
            for (date, amount) in cashflow.iter() {
                if date.date() <= pricing_date.date() {
                    continue;
                }
                diff = cal.year_fraction(
                    &min_cashflow_date,
                    &date, 
                    &daycounter)?;
                disc_factor = 1.0 / (1.0 + effective_yield).powf(diff * freq);
                res += amount * disc_factor;
            }
        }

        if !bond.is_coupon_strip()? {
            if maturity.date() > pricing_date.date() {
                diff = cal.year_fraction(
                    &min_cashflow_date, &maturity, &daycounter
                )?;

                disc_factor = 1.0 / (1.0 + effective_yield).powf(diff * freq);
                res += disc_factor;
            }
        }
        
        // 금융투자회사의 영업 및 업무에 관한 규정 별표 14
        // d = days from pricing_date to the next coupon dates
        let d = (min_cashflow_date.date() - pricing_date.date()).whole_days();
        
        // previous coupon date (or issue date)
        let previous_date = cashflow
            .keys()
            .filter(|&date| date.date() <= pricing_date.date())
            .max()
            .unwrap_or(bond.get_issue_date()?);
        // b = days from previous_date to the next coupon date
        let b = (min_cashflow_date.date() - previous_date.date()).whole_days();
        let frac = d as Real / b as Real;
        
        res /= 1.0 + effective_yield * frac;

        Ok(res)
    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;
    use crate::evaluation_date;
    use crate::instruments::fixed_coupon_bond::FixedCouponBond;
    use crate::time::{calendar::Calendar, jointcalendar::JointCalendar};
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::assets::currency::Currency;
    use crate::enums::{IssuerType, CreditRating, RankType};
    use time::Duration;

    #[test]
    fn test_krx_yield_pricer() -> Result<()> {
        let dt = datetime!(2024-03-18 16:30:00 +09:00);
        let eval_date = evaluation_date::EvaluationDate::new(dt);
        let eval_date_rc = Rc::new(RefCell::new(eval_date));
        let pricing_date = dt + Duration::days(1);
        //

        let issuedate2 = datetime!(2022-12-10 16:30:00 +09:00);
        let maturity2 = datetime!(2025-12-10 16:30:00 +09:00);
        let issuer_name2 = "Korea Gov";
        let bond_name2 = "국고채권 04250-2512(22-13)";
        let bond_code2 = "KR103501GCC0";
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;

        let bond_currency2 = Currency::KRW;
        let issuer_type2 = IssuerType::Government;
        let credit_rating2 = CreditRating::None;

        let bond = FixedCouponBond::new_from_conventions(
            bond_currency2,
            issuer_type2,
            credit_rating2,     
            RankType::Senior, 
            false, 
            0.0425, 
            10_000.0, 
            issuedate2.clone(), 
            issuedate2.clone(),
            maturity2,
            None,
            Some(pricing_date),
            calendar,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted, 
            PaymentFrequency::SemiAnnually, 
            issuer_name2.to_string(), 
            0, 
            bond_name2.to_string(), 
            bond_code2.to_string(),
        )?;
        let inst = Instrument::FixedCouponBond(bond.clone());
        let bond_yield = 0.03390;
        
        let pricer = KrxYieldPricer::new(
            bond_yield, 
            None,
            eval_date_rc.clone()
        );
        let npv = pricer.npv(&inst)?;
        let expected_npv = 1.025838;
        println!("npv: {}", npv);
        assert!(
            (npv - expected_npv).abs() < 1.0e-5,
            "npv: {}, expected_npv: {}", npv, expected_npv);

        // find bond yield
        let calc_yield = pricer.find_bond_yield(bond, npv)?;
        let expected_yield = 0.03390;
        println!("calc yield: {:?}", calc_yield);
        assert!(
            (calc_yield - expected_yield).abs() < 1.0e-5,
            "calc yield: {}, expected_yield: {}", calc_yield, expected_yield
        );

        Ok(())
    }
}