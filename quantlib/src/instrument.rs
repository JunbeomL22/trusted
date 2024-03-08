use crate::definitions::Real;
use crate::assets::currency::Currency;
use time::OffsetDateTime;
use std::ops::Index;
use crate::pricing_engines::match_parameter::MatchPrameter;
use crate::parameters::rate_index::RateIndex;
use crate::enums::{IssuerType, CreditRating, RankType, AccountingLevel};
use std::sync::Arc;

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
    fn get_underlying_codes(&self) -> &Vec<String> { vec![].as_ref() }
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
    // There may be an instrument that does not need discount curve, e.g., Futures
    //fn get_discount_curve_name(&self) -> &String { &"Dummy".to_string() }
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
    instruments: Vec<Arc<Instrument>>,
    match_parameter: Arc<MatchPrameter>,
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
            match_parameter: Arc::new(MatchPrameter::default()),
        }
    }
}

impl Instruments {
    pub fn iter(&self) -> std::slice::Iter<'_, Arc<Instrument>> {
        self.instruments.iter()
    }

    pub fn new(
        instruments: Vec<Arc<Instrument>>, 
        match_parameter: Arc<MatchPrameter>) -> Instruments 
        {
        Instruments { 
            instruments,
            match_parameter,
        }
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn get_instruments_clone(&self) -> Vec<Arc<Instrument>> {
        let mut res = Vec::<Arc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            res.push(instrument.clone());
        }
        res
    }

    pub fn get_all_underlying_codes(&self) -> &Vec<String> {
        let mut underlying_codes = Vec::<String>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.as_trait().get_underlying_codes();
            for name in names.iter() {
                if !underlying_codes.contains(&name) {
                    underlying_codes.push(name.clone());
                }
            }
        }
        &underlying_codes
    }
    
    pub fn instruments_with_underlying(&self, und_code: &String) -> Vec<Arc<Instrument>> {
        let mut res = Vec::<Arc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.as_trait().get_underlying_codes();
            if names.contains(und_code) {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_currency(&self, currency: &Currency) -> Vec<Arc<Instrument>> {
        let mut res = Vec::<Arc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.as_trait().get_currency() == currency {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_type(&self, type_name: &'static str) -> Vec<Arc<Instrument>> {
        let mut res = Vec::<Arc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.as_trait().get_type_name() == type_name {
                res.push(instrument);
            }
        }
        res
    }

    pub fn instruments_using_curve(&self, curve_name: &String) -> Vec<Arc<Instrument>> {
        let mut res = Vec::<Arc<Instrument>>::new();
        // 1) discount curve
        // 2) collateral curves
        // 3) borrowing curves
        // 4) rate index forward curves
        for instrument in self.instruments.iter() {
            // 1)
            if self.match_parameter.get_discount_curve_name(instrument) == curve_name {
                res.push(instrument.clone());
            }
            // 2)
            if self.match_parameter.get_collateral_curve_names(instrument).contains(&curve_name) {
                res.push(instrument.clone());
            }
            // 3)
            if self.match_parameter.get_borrowing_curve_names(instrument).contains(&curve_name) {
                res.push(instrument.clone());
            }
        }
        res
    }
}