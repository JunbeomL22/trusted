use crate::instruments::schedule::Schedule;
use crate::instruments::{
    bond::Bond,
    stock_futures::StockFutures,
    plain_swap::PlainSwap,
    bond_futures::BondFutures,
    ktbf::KTBF,
};
use crate::definitions::Real;
use crate::assets::currency::Currency;
use crate::pricing_engines::match_parameter::MatchParameter;
use crate::parameters::{
    rate_index::RateIndex,
    zero_curve::ZeroCurve,
};
use crate::enums::{IssuerType, CreditRating, RankType, AccountingLevel};
use crate::time::{
    conventions::PaymentFrequency,
    jointcalendar::JointCalendar,
};
use crate::data::history_data::CloseData;
// 
use anyhow::{Result, anyhow};
use enum_dispatch::enum_dispatch;
use std::{
    rc::Rc,
    cell::RefCell,
    ops::Index,
    collections::HashMap,
};
use time::OffsetDateTime;


#[enum_dispatch]
pub trait InstrumentTrait{
    // The following methods are mandatory for all instruments
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
    fn get_unit_notional(&self) -> Real;
    fn get_type_name(&self) -> &'static str;
    fn get_average_trade_price(&self) -> Real { 0.0 }
    //
    fn get_accountring_level(&self) -> AccountingLevel { AccountingLevel::Level1 }
    //
    // There is an instrument that does not have maturity date, so it is optional
    fn get_maturity(&self) -> Option<&OffsetDateTime> { None }
    // There is an instrument that does not have underlying names, 
    // so the default action is to return an empty vector
    fn get_underlying_codes(&self) -> Vec<&String> { vec![] }
    // only for bonds, so None must be allowed
    fn get_credit_rating(&self) -> Result<&CreditRating> {
        Err(anyhow!("({}:{}) not supported instrument type on get_credit_rating", file!(), line!()))
    }
    // only for bonds, so None must be allowed
    fn get_issuer_type(&self) -> Result<&IssuerType> {
        Err(anyhow!("({}:{}) not supported instrument type on get_issuer_type", file!(), line!()))
    }
    // only for bonds, so None must be allowed
    fn get_rank_type(&self) -> Result<&RankType> {
        Err(anyhow!("({}:{}) not supported instrument type on get_rank_type", file!(), line!()))
    }
    // only for bonds, so None must be allowed
    fn get_issuer_name(&self) -> Result<&String> {
        Err(anyhow!("({}:{}) not supported instrument type on get_issuer_name", file!(), line!()))
    }

    // only for FloatingRateNote, IRS, OIS, and other swaps
    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        Err(anyhow!("({}:{}) not supported instrument type on get_rate_index", file!(), line!()))
    }
    
    fn get_cashflows(
        &self, 
        _pricing_date: &OffsetDateTime,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        _past_data: Option<Rc<CloseData>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> { 
        Err(anyhow!("not supported instrument type on get_coupon_cashflow"))
    }

    fn get_floating_cashflows(
        &self, 
        _pricing_date: &OffsetDateTime,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>,
        _past_data: Option<Rc<CloseData>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> { 
        Err(anyhow!("not supported instrument type on get_floating_cashflows"))
    }

    fn get_fixed_cashflows(
        &self, 
        _pricing_date: &OffsetDateTime,
    ) -> Result<HashMap<OffsetDateTime, Real>> { 
        Err(anyhow!("not supported instrument type on get_fixed_cashflows"))
    }

    fn get_pricing_date(&self) -> Result<Option<&OffsetDateTime>, anyhow::Error> {
        Err(anyhow!("not supported instrument type on get_pricing_date"))
    }

    fn is_coupon_strip(&self) -> Result<bool> { 
        Err(anyhow!("not supported instrument type on is_coupon_strip"))
    }

    fn get_underlying_bonds(&self) -> Result<&Vec<Bond>> {
        Err(anyhow!("not supported instrument type on get_underlying_bonds"))
    }
    
    fn get_coupon_frequency(&self) -> Result<PaymentFrequency> {
        Err(anyhow!("not supported instrument type on get_frequency"))
    }

    fn get_calendar(&self) -> Result<&JointCalendar> {
        Err(anyhow!("not supported instrument type on get_calendar"))
    }

    fn get_issue_date(&self) -> Result<&OffsetDateTime> {
        Err(anyhow!("not supported instrument type on issue_date"))
    }

    fn get_virtual_bond_npv(&self, _bond_yield: Real) -> Result<Real> {
        Err(anyhow!("not supported instrument type on get_virtual_bond_npv"))
    }

    fn get_schedule(&self) -> Result<&Schedule> {
        Err(anyhow!("not supported instrument type on get_schedule"))
    }

    fn get_fixed_leg_currency(&self) -> Result<&Currency> {
        Err(anyhow!("not supported instrument type on get_fixed_leg_currency"))
    }

    fn get_floating_leg_currency(&self) -> Result<&Currency> {
        Err(anyhow!("not supported instrument type on get_floating_leg_currency"))
    }

}

#[enum_dispatch(InstrumentTrait)]
#[derive(Clone, Debug)]
pub enum Instrument {
    StockFutures(StockFutures),
    Bond(Bond),
    BondFutures(BondFutures),
    KTBF(KTBF),
    PlainSwap(PlainSwap),
}

/// calculation groups for calculation optimization, 
/// On the group, again select calculation sets based on currency and underlying assets (not sub|superset, exact the same assets)
/// currency and underlying_assets categorization
/// GROUP1: Vec<&'static str> = vec!["StockFutures"]; 
/// GROUP2: Vec<&'static str> = vec!["FixedCouponBond", "BondFutures", "KTBF"]; 
/// GROUP3: Vec<&'static str> = vec!["StructuredProduct"]; 
pub struct Instruments {
    instruments: Vec<Rc<Instrument>>,
}

impl Index<usize> for Instruments {
    type Output = Instrument;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instruments[index]
    }
}

impl Default for Instruments {
    fn default() -> Self {
        Instruments { 
            instruments: vec![],
        }
    }
}

impl Instruments {
    pub fn iter(&self) -> std::slice::Iter<'_, Rc<Instrument>> {
        self.instruments.iter()
    }

    pub fn new(instruments: Vec<Rc<Instrument>>) -> Instruments {
        Instruments { 
            instruments,
        }
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn get_instruments_clone(&self) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            res.push(instrument.clone());
        }
        res
    }

    pub fn get_all_underlying_codes(&self) -> Vec<&String> {
        let mut underlying_codes = Vec::<&String>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.get_underlying_codes();
            for name in names.iter() {
                if !underlying_codes.contains(&name) {
                    underlying_codes.push(name);
                }
            }
        }
        underlying_codes
    }

    pub fn get_all_type_names(&self) -> Vec<&'static str> {
        let mut type_names = Vec::<&'static str>::new();
        for instrument in self.instruments.iter() {
            let name = instrument.get_type_name();
            if !type_names.contains(&name) {
                type_names.push(name);
            }
        }
        type_names
    }

    pub fn get_all_currencies(&self) -> Vec<&Currency> {
        let mut currencies = Vec::<&Currency>::new();
        for instrument in self.instruments.iter() {
            let currency = instrument.get_currency();
            if !currencies.contains(&currency) {
                currencies.push(currency);
            }
        }
        currencies
    }
    
    pub fn instruments_with_underlying(
        &self, 
        und_code: &String,
    ) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.get_underlying_codes();
            if names.contains(&und_code) {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_currency(&self, currency: &Currency) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.get_currency() == currency {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_type(&self, type_name: &'static str) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.get_type_name() == type_name {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_using_curve(
        &self, 
        curve_name: &String,
        match_parameter: &MatchParameter,
    ) -> Result<Vec<Rc<Instrument>>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        // 1) discount curve
        // 2) collateral curves
        // 3) rate index forward curves
        // borrowing curve can not be hedged, so it skips
        for instrument in self.instruments.iter() {
            // 1)
            if match_parameter.get_discount_curve_name(instrument)? == curve_name {
                res.push(instrument.clone());
            }
            // 2)
            if match_parameter.get_collateral_curve_names(instrument)?.contains(&curve_name) {
                res.push(instrument.clone());
            }
            // 3) forward curve
            if match_parameter.get_rate_index_curve_name(instrument)? == curve_name {
                res.push(instrument.clone());
            }
        }
        Ok(res)
    }

    // all curve names including discount, collateral, and rate index forward curves
    pub fn get_all_curve_names<'a>(&'a self, match_parameter: &'a MatchParameter) -> Result<Vec<&String>> {
        let mut res = Vec::<&String>::new();
        for instrument in self.instruments.iter() {
            let discount_curve_name = match_parameter.get_discount_curve_name(instrument)?;
            if !res.contains(&discount_curve_name) && discount_curve_name != "Dummy" {
                res.push(discount_curve_name);
            }
            let collateral_curve_names = match_parameter.get_collateral_curve_names(instrument)?;
            for name in collateral_curve_names.iter() {
                if !res.contains(name) && *name != "Dummy"{
                    res.push(name);
                }
            }
            let rate_index_curve_name = match_parameter.get_rate_index_curve_name(instrument)?;
            if !res.contains(&rate_index_curve_name) && rate_index_curve_name != "Dummy" {
                res.push(rate_index_curve_name);
            }
        }
        Ok(res)
    }

    pub fn instruments_with_maturity_upto(
        &self, 
        instruments: Option<&Vec<Rc<Instrument>>>,
        maturity: &OffsetDateTime
    ) -> Vec<Rc<Instrument>> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if m <= maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            },
            None => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in self.instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if m <= maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
        }
    }

    pub fn instruments_with_maturity_over(
        &self, 
        instruments: Option<&Vec<Rc<Instrument>>>,
        maturity: &OffsetDateTime
    ) -> Vec<Rc<Instrument>> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in instruments.iter() {
                    if instrument.get_maturity() == None {
                        res.push(instrument.clone());
                    }
                    
                    if let Some(m) = instrument.get_maturity() {
                        if m > maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            },
            None => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in self.instruments.iter() {
                    if instrument.get_maturity() == None {
                        res.push(instrument.clone());
                    }
                    
                    if let Some(m) = instrument.get_maturity() {
                        if m > maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
        }
    }

    /// This method return the shortest maturity of the given instruments
    /// If instruments is None, it gives the shortest maturity of all instruments
    /// None maturity is taken as an infinite maturity
    /// Therefore, if there is no maturity, it is considered as the longest maturity
    pub fn get_shortest_maturity(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Option<OffsetDateTime> {
        match instruments {
            Some(instruments) => {
                let mut shortest_maturity: Option<OffsetDateTime> = None;
                for instrument in instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if let Some(sm) = shortest_maturity {
                            if *m < sm {
                                shortest_maturity = Some(*m);
                            }
                        } else {
                            shortest_maturity = Some(*m);
                        }
                    }
                }
                shortest_maturity
            },
            None => {
                let mut shortest_maturity: Option<OffsetDateTime> = None;
                for instrument in self.instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if let Some(sm) = shortest_maturity {
                            if *m < sm {
                                shortest_maturity = Some(*m);
                            }
                        } else {
                            shortest_maturity = Some(*m);
                        }
                    }
                }
                shortest_maturity
            }
        }
    }

    /// This method return the longest maturity of the given instruments
    /// If instruments is None, it gives the longest maturity of all instruments
    /// None maturity is taken as an infinite maturity
    /// Therefore, if there is no maturity, it is considered as the longest maturity
    pub fn get_longest_maturity(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Option<OffsetDateTime> {
        match instruments {
            Some(instruments) => {
                let mut longest_maturity: Option<OffsetDateTime> = None;
                for instrument in instruments.iter() {
                    match instrument.get_maturity() {
                        Some(m) => {
                            if let Some(sm) = longest_maturity {
                                if *m > sm {
                                    longest_maturity = Some(*m);
                                }
                            } else {
                                longest_maturity = Some(*m);
                            }
                        },
                        None => {
                            longest_maturity = None;
                            break;
                        }
                    }
                }
                longest_maturity
            },
            None => {
                let mut longest_maturity: Option<OffsetDateTime> = None;
                for instrument in self.instruments.iter() {
                    match instrument.get_maturity() {
                        Some(m) => {
                            if let Some(sm) = longest_maturity {
                                if *m > sm {
                                    longest_maturity = Some(*m);
                                }
                            } else {
                                longest_maturity = Some(*m);
                            }
                        },
                        None => {
                            longest_maturity = None;
                            break;
                        }
                    }
                }
                longest_maturity
            }
        }
    }

    pub fn get_all_inst_code_clone(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Vec<String> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<String>::new();
                for instrument in instruments.iter() {
                    res.push(instrument.get_code().clone());
                }
                res
            },
            None => {
                let mut res = Vec::<String>::new();
                for instrument in self.instruments.iter() {
                    res.push(instrument.get_code().clone());
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::assets::currency::Currency;
    use crate::instruments::stock_futures::StockFutures;
    use crate::instruments::plain_swap::PlainSwap;
    use time::macros::datetime;
    use crate::parameters::rate_index::RateIndex;
    use crate::enums::RateIndexCode;
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use crate::time::{
        jointcalendar::JointCalendar, 
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        calendar::Calendar,
    };
    use anyhow::Result;
    
    #[test]
    fn test_instruments() -> Result<()> {
        let fut1 = StockFutures::new(
            340.0,
            datetime!(2022-01-01 09:00:00 UTC),
            datetime!(2022-12-01 09:00:00 UTC),
            datetime!(2022-12-01 09:00:00 UTC),
            datetime!(2022-12-01 09:00:00 UTC),
            250_000.0,
            Currency::KRW,
            Currency::KRW,
            "KOSPI2".to_string(),
            "KOSPI2 FUT".to_string(),
            "165XXX".to_string(),
        );

        let fut2 = StockFutures::new(
            5000.0,
            datetime!(2022-12-01 09:00:00 UTC),
            datetime!(2024-03-01 09:00:00 UTC),
            datetime!(2024-03-01 09:00:00 UTC),
            datetime!(2024-03-01 09:00:00 UTC),
            50.0,
            Currency::USD,
            Currency::USD,
            "SPX".to_string(),
            "SPX FUT".to_string(),
            "ESH24".to_string(),
        );

        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk = Calendar::SouthKorea(sk);
        let joint_calendar = JointCalendar::new(vec![sk])?;

        let rate_index = RateIndex::new(
            String::from("91D"),
            Currency::KRW,
            RateIndexCode::CD,
            "CD91".to_string(),
        )?;

        // make SouthKorea(SouthKorea::Settlement) JointCalendar
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk = Calendar::SouthKorea(sk);
        let joint_calendar = JointCalendar::new(vec![sk])?;

        let issue_date = datetime!(2021-01-01 09:00:00 UTC);
        let maturity_date = datetime!(2021-12-31 09:00:00 UTC);
        let irs = PlainSwap::new_from_conventions(
            Currency::KRW,
            Currency::KRW,
            //
            None, None, None, None,
            //
            10_000_000_000.0,
            issue_date.clone(),
            issue_date.clone(),
            maturity_date.clone(),
            //
            Some(0.02),
            Some(rate_index.clone()),
            None,
            //
            true,
            DayCountConvention::Actual365Fixed,
            DayCountConvention::Actual365Fixed,
            BusinessDayConvention::ModifiedFollowing,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            PaymentFrequency::Quarterly,
            //
            1,
            0,
            //
            joint_calendar,
            "KRW IRS".to_string(),
            "KRW IRS code".to_string(),
        )?;

        // make Instrument using fut1, fut2, irs
        let instruments = Instruments::new(vec![
            Rc::new(Instrument::StockFutures(fut1.clone())),
            Rc::new(Instrument::StockFutures(fut2.clone())),
            Rc::new(Instrument::PlainSwap(irs.clone())),
        ]);

        // make MatchParameter
        let mut collateral_curve_map = HashMap::<String, String>::new();
        let mut rate_index_curve_map = HashMap::<RateIndexCode, String>::new();
        let borrowing_curve_map = HashMap::<String, String>::new();
        let bond_curve_map = HashMap::<(
            String, 
            IssuerType, 
            CreditRating, 
            Currency
        ), String>::new();

        // "KOSPI2" -> "KRWGOV"
        // "SPX" -> "USGOV"
        // RateIndexCode::CD -> "KRWIRS"
        collateral_curve_map.insert("KOSPI2".to_string(), "KRWGOV".to_string());
        collateral_curve_map.insert("SPX".to_string(), "USGOV".to_string());
        rate_index_curve_map.insert(RateIndexCode::CD, "KRWIRS".to_string());

        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_curve_map,
            rate_index_curve_map,
        );

        // test get_all_underlying_codes
        let underlying_codes = instruments.get_all_underlying_codes();
        assert_eq!(underlying_codes, vec![&"KOSPI2".to_string(), &"SPX".to_string()]);
        // test instruments_with_underlying
        let instruments_with_kospi2 = instruments.instruments_with_underlying(
            &"KOSPI2".to_string(),
        );

        assert_eq!(fut1.get_code(), instruments_with_kospi2[0].get_code());
        assert_eq!(fut1.get_name(), instruments_with_kospi2[0].get_name());
        assert_eq!(fut1.get_currency(), instruments_with_kospi2[0].get_currency());

        // test get_all_curve_names
        let all_curve_names = instruments.get_all_curve_names(&match_parameter)?;
        assert_eq!(all_curve_names, vec![&"KRWGOV", &"USGOV", &"KRWIRS"]);
        // test instruments_using_curve
        let instruments_using_krw_gov = instruments.instruments_using_curve(
            &"KRWGOV".to_string(),
            &match_parameter,
        )?;

        assert_eq!(fut1.get_code(), instruments_using_krw_gov[0].get_code());


        // test discount curve
        let instruments_using_krw_irs = instruments.instruments_using_curve(
            &"KRWIRS".to_string(),
            &match_parameter,
        )?;

        assert_eq!(irs.get_code(), instruments_using_krw_irs[0].get_code());

        // test instruments_with_currency
        let instruments_with_krw = instruments.instruments_with_currency(&Currency::KRW);
        assert_eq!(fut1.get_code(), instruments_with_krw[0].get_code());
        assert_eq!(irs.get_code(), instruments_with_krw[1].get_code());

        // test instruments_with_type
        let instruments_with_stock_futures = instruments.instruments_with_type("StockFutures");
        assert_eq!(fut1.get_code(), instruments_with_stock_futures[0].get_code());
        assert_eq!(fut2.get_code(), instruments_with_stock_futures[1].get_code());

        let instruments_with_irs = instruments.instruments_with_type("PlainSwap");
        assert_eq!(irs.get_code(), instruments_with_irs[0].get_code());

        // test instruments_with_maturity_upto
        let instruments_with_maturity_upto = instruments.instruments_with_maturity_upto(
            None,
            &datetime!(2022-12-01 09:00:00 UTC)
        );
        assert_eq!(fut1.get_code(), instruments_with_maturity_upto[0].get_code());
        assert_eq!(irs.get_code(), instruments_with_maturity_upto[1].get_code());

        Ok(())
    }
}