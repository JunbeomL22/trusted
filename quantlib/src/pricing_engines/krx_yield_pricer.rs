use crate::definitions::Real;
use crate::instruments::bond::Bond;
use crate::pricing_engines::pricer::PricerTrait;
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::{Instrument, InstrumentTrait};
use crate::evaluation_date::EvaluationDate;
use crate::time::{
    conventions::DayCountConvention,
    calendar_trait::CalendarTrait,
};
use crate::parameters::zero_curve::ZeroCurve;
use crate::data::history_data::CloseData;
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
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    daycount: DayCountConvention,
    forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
    past_fixing_data: Option<Rc<CloseData>>,
}

impl KrxYieldPricer {
    pub fn new(
        bond_yield: Real, 
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        past_fixing_data: Option<Rc<CloseData>>,
    ) -> KrxYieldPricer {
        KrxYieldPricer { 
            bond_yield,
            evaluation_date,
            daycount: DayCountConvention::StreetConvention,
            forward_curve,
            past_fixing_data,
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
        bond: Bond, 
        npv: Real,
        init_guess: Option<Real>,
    ) -> Result<Real> {
        let pricer = self.clone();
        let problem = KrxYieldPricerCostFunction::new(bond, npv, pricer);
        let linesearch = MoreThuenteLineSearch::new();        
        
        let solver = SteepestDescent::new(linesearch);
        let init_param = init_guess.unwrap_or(0.02);
        let executor = Executor::new(problem, solver)
            .configure(|state|
                state
                    .param(init_param)
                    .max_iters(30)
                    .target_cost(1.0e-9)
            );
        
        let res = executor.run()?;
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
    pub fn new(bond: Bond, npv: Real, pricer: KrxYieldPricer) -> KrxYieldPricerCostFunction {
        KrxYieldPricerCostFunction {
            bond: Instrument::Bond(bond),
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
        let h = 5.0e-5;
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
        let freq = bond.get_coupon_frequency()?.as_real();
        let effective_yield = self.bond_yield / freq;
        let cal = bond.get_calendar()?;
        let mut diff: Real;
        let daycounter = self.daycount;
        let maturity = bond.get_maturity().unwrap();

        let cashflow = bond.get_cashflows(
            &pricing_date,
            self.forward_curve.clone(),
            self.past_fixing_data.clone()
        ).with_context(|| anyhow!(
            "{}:{} (KrxYieldPricer) Failed to get coupon cashflow of {} ({})",
            file!(), line!(), bond.get_name(), bond.get_code()
        ))?;

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
        
        // 금융투자회사의 영업 및 업무에 관한 규정 별표 14
        // d = days from pricing_date to the next coupon dates
        let d = (min_cashflow_date.date() - pricing_date.date()).whole_days();
        
        // previous coupon date (or issue date)
        let schedule = bond.get_schedule()?;

        let previous_date = schedule
            .iter()
            .filter(|&base_schedule| base_schedule.get_payment_date().date() <= pricing_date.date())
            .map(|base_schedule| base_schedule.get_payment_date())
            .max()
            .unwrap_or(bond.get_issue_date()?);
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
    use crate::instruments::bond::Bond;
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

        let bond = Bond::new_from_conventions(
            issuer_type2, 
            credit_rating2, 
            issuer_name2.to_string(), 
            RankType::Senior,
            bond_currency2,
            //
            1_000_0.0,
            false, 
            //
            issuedate2.clone(), 
            issuedate2.clone(),
            Some(pricing_date.clone()),
            maturity2,
            //
            Some(0.0425), 
            None,
            None,
            None,
            //
            calendar,
            //
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted, 
            PaymentFrequency::SemiAnnually, 
            //
            0,
            0,  
            bond_name2.to_string(), 
            bond_code2.to_string(),
        )?;

        let cashflows = bond.get_cashflows(
            &pricing_date,
            None,
            None
        )?;

        for (date, amount) in cashflows.iter() {
            println!("date: {}, amount: {}", date, amount);
        }

        let inst = Instrument::Bond(bond.clone());
        let bond_yield = 0.03390;
        
        let pricer = KrxYieldPricer::new(
            bond_yield, 
            eval_date_rc.clone(),
            None,
            None,
        );

        let npv = pricer.npv(&inst)?;
        let expected_npv = 1.025838;
        println!("npv: {}", npv);
        assert!(
            (npv - expected_npv).abs() < 1.0e-5,
            "npv: {}, expected_npv: {}", npv, expected_npv);

        // find bond yield
        let calc_yield = pricer.find_bond_yield(bond, npv, Some(-0.1))?;
        let expected_yield = 0.03390;
        println!("calc yield: {:?}", calc_yield);
        assert!(
            (calc_yield - expected_yield).abs() < 1.0e-5,
            "calc yield: {}, expected_yield: {}", calc_yield, expected_yield
        );

        Ok(())
    }
}