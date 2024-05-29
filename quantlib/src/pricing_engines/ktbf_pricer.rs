use crate::parameters::zero_curve::ZeroCurve;
use crate::evaluation_date::EvaluationDate;
use crate::pricing_engines::{
    npv_result::NpvResult, 
    pricer::PricerTrait,
    krx_yield_pricer::KrxYieldPricer,
    bond_pricer::BondPricer,
};
use crate::instrument::{
    Instrument,
    InstrumentTrait,
};
use crate::definitions::Real;
use crate::enums::Compounding;
//
use anyhow::Result;
use std::{
    rc::Rc, 
    cell::RefCell,
};

pub struct KtbfPricer {
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    discount_curve: Rc<RefCell<ZeroCurve>>,
    borrowing_curve: Rc<RefCell<ZeroCurve>>,
}

impl KtbfPricer {
    pub fn new(
        evaluation_date: Rc<RefCell<EvaluationDate>>,
        discount_curve: Rc<RefCell<ZeroCurve>>,
        borrowing_curve: Rc<RefCell<ZeroCurve>>,
    ) -> KtbfPricer {
        KtbfPricer {
            evaluation_date,
            discount_curve,
            borrowing_curve,
        }
    }
}

impl PricerTrait for KtbfPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        let bond_pricer = BondPricer::new(
            self.evaluation_date.clone(),
            self.discount_curve.clone(),
            None,
            None,
        );
        
        let mut bond_yields = Vec::new();

        let underlying_bonds = instrument.get_underlying_bonds()?;

        let krx_yield_pricer = KrxYieldPricer::new(
            self.evaluation_date.clone(),
            0.0,
            None,
            None,
        );

        let init_guess = self.discount_curve.borrow().get_forward_rate_from_evaluation_date(
            underlying_bonds[0].get_maturity().unwrap(),
            Compounding::Simple,
        )?;

        for bond in underlying_bonds.iter() {
            let inst = Instrument::Bond(bond.clone());
            let npv = bond_pricer.npv(&inst)?;
            let yield_ = krx_yield_pricer.find_bond_yield(
                bond.clone(),
                npv,
                Some(init_guess)
            )?;
            bond_yields.push(yield_);
        }

        let average_yield = bond_yields.iter().sum::<Real>() / bond_yields.len() as Real;

        let mut ktbf_price = instrument.get_virtual_bond_npv(average_yield)?;

        let borrowing_cost = self.borrowing_curve.borrow().get_discount_factor_at_date(
            instrument.get_maturity().unwrap(),
        )?;

        ktbf_price *= borrowing_cost;

        Ok(ktbf_price)

    }

    fn npv_result(&self, instrument: &Instrument) -> Result<NpvResult> {
        let npv = self.npv(instrument)?;
        Ok(NpvResult::new_from_npv(npv))
    }

}

#[cfg(test)]
mod tests {
    use crate::instrument::Instrument;
    use crate::instruments::bond::Bond;
    use crate::instruments::ktbf::{KtbfVirtualBond, KTBF};
    use crate::evaluation_date::EvaluationDate;
    use crate::parameters::zero_curve::ZeroCurve;
    use crate::data::vector_data::VectorData;
    use crate::currency::Currency;
    use crate::enums::{
        IssuerType,
        CreditRating,
        RankType,
    };
    use crate::time::{
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        jointcalendar::JointCalendar,
        conventions::{BusinessDayConvention, PaymentFrequency, DayCountConvention},
        calendar::Calendar,
    };
    use crate::pricing_engines::{
        ktbf_pricer::KtbfPricer,
        pricer::{PricerTrait, Pricer},
    };
    //
    use time::macros::datetime;
    use std::rc::Rc;
    use std::cell::RefCell;
    use ndarray::array;
    use time::Duration;
    use anyhow::Result;
    
    #[test]
    fn test_ktbf_pricer() -> Result<()> {
        let eval_date = datetime!(2024-01-02 00:00:00 UTC);
        let evaluation_date = Rc::new(RefCell::new(EvaluationDate::new(eval_date)));
        let curve_data = VectorData::new(
            array![0.030, 0.040],
            None,
            Some(array![0.5, 5.0]),
            None,//eval_date.clone(),
            Currency::KRW,
            "KRWGOV".to_string(),
        )?;
        let discount_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &curve_data,
            "KRWGOV".to_string(),
            "KRWGOV".to_string(),
        )?;

        let borrowing_curve_data = VectorData::new(
            array![0.003],
            None,
            Some(array![0.5]),
            None,//eval_date.clone(),
            Currency::KRW,
            "KTBF3Y".to_string(),
        )?;
        let borrowing_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &borrowing_curve_data,
            "KTBF3Y".to_string(),
            "KTBF3Y".to_string(),
        )?;
        let sk = Calendar::SouthKorea(SouthKorea::new(SouthKoreaType::Settlement));
        let calendar = JointCalendar::new(vec![sk])?;
        let ktbf_maturity = eval_date + Duration::days(90);
        // make two bonds whose maturity is 3 year and 5 year from the evaluation date
        // both are fixed coupon bond which 3% coupon rate
        // make it from crate::instruments::bond::Bond::new_from_convention
        let issue_date = datetime!(2024-01-02 00:00:00 UTC);
        let maturity = datetime!(2027-01-02 00:00:00 UTC);
        let bond1 = Bond::new_from_conventions(
            IssuerType::Government,
            CreditRating::None,
            "Korea Government".to_string(),
            RankType::Senior,
            Currency::KRW,
            //
            10_000.0,
            false,
            //
            issue_date.clone(),
            issue_date.clone(),
            Some(ktbf_maturity.clone()),
            maturity,
            //
            Some(0.03),
            None,
            None,
            None,
            //
            calendar.clone(),
            //
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
            //
            "Bond1".to_string(),
            "Bond1".to_string(),
        )?;

        let maturity = datetime!(2029-01-02 00:00:00 UTC);
        let bond2 = Bond::new_from_conventions(
            IssuerType::Government,
            CreditRating::None,
            "Korea Government".to_string(),
            RankType::Senior,
            Currency::KRW,
            //
            10_000.0,
            false,
            //
            issue_date.clone(),
            issue_date.clone(),
            Some(ktbf_maturity.clone()),
            maturity,
            //
            Some(0.03),
            None,
            None,
            None,
            //
            calendar.clone(),
            //
            true,
            DayCountConvention::StreetConvention,
            BusinessDayConvention::Unadjusted,
            PaymentFrequency::SemiAnnually,
            0,
            0,
            //
            "Bond2".to_string(),
            "Bond2".to_string(),
        )?;

        let virtual_bond = KtbfVirtualBond::new(
            3,
            0.05,
            PaymentFrequency::SemiAnnually,
            100.0
        );

        let ktbf = KTBF::new(
            Currency::KRW,
            1_000_000.0,
            issue_date.clone(),
            ktbf_maturity.clone(),
            ktbf_maturity.clone(),
            virtual_bond,
            vec![bond1, bond2],
            "KTBF3Y".to_string(),
            "KTBF".to_string(),
            "KTBF".to_string(),
        )?;

        let ktbf_pricer = KtbfPricer::new(
            evaluation_date.clone(),
            Rc::new(RefCell::new(discount_curve)),
            Rc::new(RefCell::new(borrowing_curve)),
        );

        let pricer = Pricer::KtbfPricer(ktbf_pricer);
        let npv = pricer.npv(&Instrument::KTBF(ktbf))?;
        println!("KTBF NPV: {}", npv);


        Ok(())
    }
}