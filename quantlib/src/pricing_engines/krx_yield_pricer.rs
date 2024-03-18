use crate::definitions::{Real, Integer};
use crate::instruments::fixed_coupon_bond::FixedCouponBond;
use crate::pricing_engines::pricer::PricerTrait;
use crate::time::calendars::calendar_trait::CalendarTrait;
use crate::time::conventions::DayCountConvention;
use crate::pricing_engines::npv_result::NpvResult;
use crate::instrument::{Instrument, InstrumentTriat};
use crate::evaluation_date::EvaluationDate;
//
use anyhow::{Result, Context, anyhow};
use time::convert::Day;
use std::rc::Rc;
use std::cell::RefCell;

/// 금융투자회사의 영업 및 업무에 관한 규정 별표 14
/// https://law.kofia.or.kr/service/law/lawFullScreenContent.do?seq=136&historySeq=263
pub struct KrxYieldPricer {
    bond_yield: Real,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    daycount: DayCountConvention,
}

impl KrxYieldPricer {
    pub fn new(
        bond_yield: Real, evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> KrxYieldPricer {
        KrxYieldPricer { 
            bond_yield,
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

    pub fn find_bond_yield(&self, bond: FixedCouponBond, price: Real) -> Result<Real> {
        Ok(0.0)
    }
}

impl PricerTrait for KrxYieldPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let bond = instrument.as_fixed_coupon_bond()?;
        let mut res: Real = 0.0;
        let mut disc_factor: Real;
        let eval_dt = self.evaluation_date.borrow().get_date_clone();
        let pricing_date = bond.get_pricing_date().unwrap_or(&eval_dt);
        let freq = bond.get_frequency().as_real();
        let effective_yield = self.bond_yield / freq;
        let cal = bond.get_calendar();
        let mut diff: Real;
        let daycounter = self.daycount;
        let maturity = instrument.get_maturity().unwrap();

        let cashflow = bond.get_coupon_cashflow(
            &pricing_date,
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

        if !bond.is_coupon_strip() {
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
            .unwrap_or(&bond.get_issue_date());
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
        let inst = Instrument::FixedCouponBond(bond);
        let bond_yield = 0.03390;
        
        let pricer = KrxYieldPricer::new(bond_yield, eval_date_rc.clone());
        let npv = pricer.npv(&inst)?;
        println!("npv: {}", npv);
        Ok(())
    }

}