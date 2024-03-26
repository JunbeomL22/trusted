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
use crate::time::calendars::nullcalendar::NullCalendar;
//
use anyhow::Result;
use std::{
    rc::Rc, 
    cell::RefCell,
};

pub struct KtbfPricer {
    discount_curve: Rc<RefCell<ZeroCurve>>,
    evaluation_date: Rc<RefCell<EvaluationDate>>,
    //time_calculator: NullCalendar,
}

impl KtbfPricer {
    pub fn new(
        discount_curve: Rc<RefCell<ZeroCurve>>,
        evaluation_date: Rc<RefCell<EvaluationDate>>,
    ) -> KtbfPricer {
        KtbfPricer {
            discount_curve,
            evaluation_date,
            //time_calculator: NullCalendar::new(),
        }
    }
}

impl PricerTrait for KtbfPricer {
    fn npv(&self, instrument: &Instrument) -> Result<Real> {
        //let mut res: Real = 0.0;
        //let mut disc_factor: Real;
        //let eval_dt = self.evaluation_date.borrow().get_date_clone();
        //let pricing_date = instrument.get_maturity().unwrap();
        
        let bond_pricer = BondPricer::new(
            self.discount_curve.clone(),
            self.evaluation_date.clone(),
            None,
            None,
        );
        
        let mut bond_yields = Vec::new();

        let underlying_bonds = instrument.get_underlying_bonds()?;

        let krx_yield_pricer = KrxYieldPricer::new(
            0.0,
            self.evaluation_date.clone(),
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

        let ktbf_price = instrument.get_virtual_bond_npv(average_yield)?;

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
    use crate::assets::currency::Currency;
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
            eval_date.clone(),
            Currency::KRW,
            "KRWGOV".to_string(),
        )?;
        let discount_curve = ZeroCurve::new(
            evaluation_date.clone(),
            &curve_data,
            "KRWGOV".to_string(),
            "KRWGOV".to_string(),
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
            "KTBF".to_string(),
            "KTBF".to_string(),
        )?;

        let ktbf_pricer = KtbfPricer::new(
            Rc::new(RefCell::new(discount_curve)),
            evaluation_date.clone(),
        );

        let pricer = Pricer::KtbfPricer(ktbf_pricer);
        let npv = pricer.npv(&Instrument::KTBF(ktbf))?;
        println!("KTBF NPV: {}", npv);


        Ok(())
    }
}