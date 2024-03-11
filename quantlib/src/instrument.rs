use crate::definitions::Real;
use crate::assets::currency::Currency;
use time::{OffsetDateTime, Duration};
use std::ops::Index;
use crate::pricing_engines::match_parameter::MatchParameter;
use crate::parameters::rate_index::RateIndex;
use crate::enums::{IssuerType, CreditRating, RankType, AccountingLevel};
use std::rc::Rc;

pub trait InstrumentTriat{
    // The following methods are mandatory for all instruments
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
    fn get_unit_notional(&self) -> Real;
    fn get_type_name(&self) -> &'static str;
    // There is an instrument that does not have maturity date, so it is optional
    fn get_maturity(&self) -> Option<&OffsetDateTime> { None }
    // There is an instrument that does not have underlying names, 
    // so the default action is to return an empty vector
    fn get_underlying_codes(&self) -> Vec<&String> { vec![] }
    // only for bonds, so None must be allowed
    fn get_credit_rating(&self) -> Option<&CreditRating> { None }
    // only for bonds, so None must be allowed
    fn get_issuer_type(&self) -> Option<&IssuerType> { None }
    // only for bonds, so None must be allowed
    fn get_rank_type(&self) -> Option<&RankType> { None }
    // only for bonds, so None must be allowed
    fn get_issuer_name(&self) -> Option<&String> { None }
    // only for instruments for floating coupon,
    // e.g., frn, irs, etc, so None must be allowed
    //fn get_rate_index_forward_curve_name(&self) -> Option<&String> { None }
    //
    fn get_average_trade_price(&self) -> Real { 0.0 }
    //
    fn get_accountring_level(&self) -> AccountingLevel { AccountingLevel::Level1 }
    //
    fn get_rate_index(&self) -> Option<&RateIndex> { None }
    // 
}

pub enum Instrument {
    StockFutures(Box<dyn InstrumentTriat>),
    FixedCouponBond(Box<dyn InstrumentTriat>),
    FloatingRateNote(Box<dyn InstrumentTriat>),
    BondFutures(Box<dyn InstrumentTriat>),
    IRS(Box<dyn InstrumentTriat>),
    KTBF(Box<dyn InstrumentTriat>),
}

impl Instrument {
    pub fn as_trait(&self) -> &(dyn InstrumentTriat) {
        match self {
            Instrument::StockFutures(instrument) => &**instrument,
            Instrument::FixedCouponBond(instrument) => &**instrument,
            Instrument::FloatingRateNote(instrument) => &**instrument,
            Instrument::BondFutures(instrument) => &**instrument,
            Instrument::IRS(instrument) => &**instrument,
            Instrument::KTBF(instrument) => &**instrument,
        }
    }
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
            let names = instrument.as_trait().get_underlying_codes();
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
            let name = instrument.as_trait().get_type_name();
            if !type_names.contains(&name) {
                type_names.push(name);
            }
        }
        type_names
    }

    pub fn get_all_currencies(&self) -> Vec<&Currency> {
        let mut currencies = Vec::<&Currency>::new();
        for instrument in self.instruments.iter() {
            let currency = instrument.as_trait().get_currency();
            if !currencies.contains(&currency) {
                currencies.push(currency);
            }
        }
        currencies
    }
    
    pub fn instruments_with_underlying(
        &self, 
        und_code: &String,
        maturity_bound: Option<OffsetDateTime>,
    ) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.as_trait().get_underlying_codes();
            if names.contains(&und_code) {
                self.push_instrument_within_maturity(&mut res, instrument.clone(), maturity_bound)
            }
        }
        res
    }

    pub fn instruments_with_currency(&self, currency: &Currency) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.as_trait().get_currency() == currency {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_type(&self, type_name: &'static str) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.as_trait().get_type_name() == type_name {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn push_instrument_within_maturity(
        &self,
        res: &mut Vec<Rc<Instrument>>,
        instrument: Rc<Instrument>,
        maturity_bound: Option<OffsetDateTime>,
    ) {
        if let Some(maturity_bound) = maturity_bound {
            if let Some(m) = instrument.as_trait().get_maturity() {
                if m <= &maturity_bound {
                    res.push(instrument.clone());
                }
            }
        } else {
            res.push(instrument.clone());
        }
    }

    pub fn instruments_using_curve(
        &self, 
        curve_name: &String,
        match_parameter: &MatchParameter,
        maturity_bound: Option<OffsetDateTime>,
    ) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        // 1) discount curve
        // 2) collateral curves
        // 3) rate index forward curves
        // borrowing curve can not be hedged, so it skips
        for instrument in self.instruments.iter() {
            // 1)
            if match_parameter.get_discount_curve_name(instrument) == curve_name {
                self.push_instrument_within_maturity(&mut res, instrument.clone(), maturity_bound);
            }
            // 2)
            if match_parameter.get_collateral_curve_names(instrument).contains(&curve_name) {
                self.push_instrument_within_maturity(&mut res, instrument.clone(), maturity_bound)
            }
            // 3) forward curve
            if match_parameter.get_rate_index_curve_name(instrument) == curve_name {
                self.push_instrument_within_maturity(&mut res, instrument.clone(), maturity_bound)
            }
        }
        res
    }

    // all curve names including discount, collateral, and rate index forward curves
    pub fn get_all_curve_names<'a>(&'a self, match_parameter: &'a MatchParameter) -> Vec<&String> {
        let mut res = Vec::<&String>::new();
        for instrument in self.instruments.iter() {
            let discount_curve_name = match_parameter.get_discount_curve_name(instrument);
            if !res.contains(&discount_curve_name) && discount_curve_name != "Dummy" {
                res.push(discount_curve_name);
            }
            let collateral_curve_names = match_parameter.get_collateral_curve_names(instrument);
            for name in collateral_curve_names.iter() {
                if !res.contains(name) && *name != "Dummy"{
                    res.push(name);
                }
            }
            let rate_index_curve_name = match_parameter.get_rate_index_curve_name(instrument);
            if !res.contains(&rate_index_curve_name) && rate_index_curve_name != "Dummy" {
                res.push(rate_index_curve_name);
            }
        }
        res
    }

    pub fn instruments_with_maturity_upto(&self, maturity: &OffsetDateTime) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if let Some(m) = instrument.as_trait().get_maturity() {
                if m <= maturity {
                    res.push(instrument.clone());
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::assets::currency::Currency;
    use crate::instruments::stock_futures::StockFutures;
    use crate::instruments::irs::IRS;
    use time::macros::datetime;
    use crate::parameters::rate_index::RateIndex;
    use crate::enums::RateIndexCode;
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use time::Duration;
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::time::calendar::{SouthKoreaWrapper, Calendar};
    use crate::time::jointcalendar::JointCalendar;
    
    #[test]
    fn test_instruments() {
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

        let rate_index = RateIndex::new(
            PaymentFrequency::Quarterly,
            BusinessDayConvention::ModifiedFollowing,
            DayCountConvention::Actual365Fixed,
            Duration::days(91),
            Currency::USD,
            RateIndexCode::CD,
            "CD91".to_string(),
        );

        // make SouthKorea(SouthKorea::Settlement) JointCalendar
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk = Calendar::SouthKorea(SouthKoreaWrapper{c: sk});
        let joint_calendar = JointCalendar::new(vec![sk]);

        let irs = IRS::new_from_conventions(
            Currency::KRW,
            10_000_000_000.0,
            datetime!(2022-12-01 09:00:00 UTC),
            datetime!(2022-12-01 09:00:00 UTC),
            0.03,
            rate_index,
            DayCountConvention::Actual365Fixed,
            BusinessDayConvention::ModifiedFollowing,
            PaymentFrequency::Quarterly,
            2,
            0,
            joint_calendar,
            "KRW IRS".to_string(),
            "KRW IRS code".to_string(),
        );

        // make Instrument using fut1, fut2, irs
        let instruments = Instruments::new(vec![
            Rc::new(Instrument::StockFutures(Box::new(fut1.clone()))),
            Rc::new(Instrument::StockFutures(Box::new(fut2.clone()))),
            Rc::new(Instrument::IRS(Box::new(irs.clone()))),
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
            None,
        );

        assert_eq!(fut1.get_code(), instruments_with_kospi2[0].as_trait().get_code());
        assert_eq!(fut1.get_name(), instruments_with_kospi2[0].as_trait().get_name());
        assert_eq!(fut1.get_currency(), instruments_with_kospi2[0].as_trait().get_currency());

        // test get_all_curve_names
        let all_curve_names = instruments.get_all_curve_names(&match_parameter);
        assert_eq!(all_curve_names, vec![&"KRWGOV", &"USGOV", &"KRWIRS"]);
        // test instruments_using_curve
        let instruments_using_krw_gov = instruments.instruments_using_curve(
            &"KRWGOV".to_string(),
            &match_parameter,
            None,
        );

        assert_eq!(fut1.get_code(), instruments_using_krw_gov[0].as_trait().get_code());


        // test discount curve
        let instruments_using_krw_irs = instruments.instruments_using_curve(
            &"KRWIRS".to_string(),
            &match_parameter,
            None,
        );

        assert_eq!(irs.get_code(), instruments_using_krw_irs[0].as_trait().get_code());

        // test instruments_with_currency
        let instruments_with_krw = instruments.instruments_with_currency(&Currency::KRW);
        assert_eq!(fut1.get_code(), instruments_with_krw[0].as_trait().get_code());
        assert_eq!(irs.get_code(), instruments_with_krw[1].as_trait().get_code());

        // test instruments_with_type
        let instruments_with_stock_futures = instruments.instruments_with_type("StockFutures");
        assert_eq!(fut1.get_code(), instruments_with_stock_futures[0].as_trait().get_code());
        assert_eq!(fut2.get_code(), instruments_with_stock_futures[1].as_trait().get_code());

        let instruments_with_irs = instruments.instruments_with_type("IRS");
        assert_eq!(irs.get_code(), instruments_with_irs[0].as_trait().get_code());

        // test instruments_with_maturity_upto
        let instruments_with_maturity_upto = instruments.instruments_with_maturity_upto(
            &datetime!(2022-12-01 09:00:00 UTC)
        );
        assert_eq!(fut1.get_code(), instruments_with_maturity_upto[0].as_trait().get_code());
        assert_eq!(irs.get_code(), instruments_with_maturity_upto[1].as_trait().get_code());
    }
}