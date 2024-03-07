use crate::definitions::Real;
use crate::assets::currency::Currency;
use time::OffsetDateTime;
use std::ops::Index;
use std::collections::HashSet;
use crate::enums::{IssuerType, CreditRating, RankType, AccountingLevel};

pub trait InstrumentTriat<'a> {
    // The following methods are mandatory for all instruments
    fn get_name(&self) -> &'a str;
    fn get_code(&self) -> &'a str;
    fn get_currency(&self) -> &Currency;
    fn get_unit_notional(&self) -> Real;
    fn get_type_name(&self) -> &'a str;
    // There may be an instrument that does not need discount curve, e.g., Futures
    fn get_discount_curve_name(&self) -> &'a str { "Dummy" }
    // There is an instrument that does not have maturity date, so it is optional
    fn get_maturity(&self) -> Option<&OffsetDateTime> { None }
    // There is an instrument that does not have underlying names, 
    // so the default action is to return an empty vector
    fn get_underlying_codes(&self) -> Vec<&'a str> { vec![] }
    // only for bonds, so None must be allowed
    fn get_credit_rating(&self) -> Option<&CreditRating> { None }
    // only for bonds, so None must be allowed
    fn get_issuer_type(&self) -> Option<&IssuerType> { None }
    // only for bonds, so None must be allowed
    fn get_rank_type(&self) -> Option<&RankType> { None }
    // only for bonds, so None must be allowed
    fn get_issuer_name(&self) -> Option<&'a str> { None }
    // only for instruments for floating coupon,
    // e.g., frn, irs, etc, so None must be allowed
    fn get_rate_forward_curve_name(&self) -> Option<&'a str> { None }
    // 
    fn get_average_trade_price(&self) -> Real { 0.0 }
    //
    fn get_accountring_level(&self) -> AccountingLevel { AccountingLevel::Level1 }
}

pub enum Instrument<'a> {
    StockFutures(Box<dyn InstrumentTriat<'a>>),
    FixedCouponBond(Box<dyn InstrumentTriat<'a>>),
    FloatingRateNote(Box<dyn InstrumentTriat<'a>>),
    BondFutures(Box<dyn InstrumentTriat<'a>>),
    IRS(Box<dyn InstrumentTriat<'a>>),
    KTBF(Box<dyn InstrumentTriat<'a>>),
}

impl<'a> Instrument<'a> {
    pub fn as_trait(&'a self) -> &'a (dyn InstrumentTriat) {
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
pub struct Instruments<'a> {
    instruments: Vec<&'a Instrument<'a>>,
}

impl<'a> Index<usize> for Instruments<'a> {
    type Output = Instrument<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instruments[index]
    }
}

impl<'a> Default for Instruments<'a> {
    fn default() -> Self {
        Instruments { instruments: vec![] }
    }
}

impl<'a> Instruments<'a> {
    pub fn iter(&self) -> std::slice::Iter<'_, &'a Instrument<'a>> {
        self.instruments.iter()
    }

    pub fn new(instruments: Vec<&'a Instrument<'a>>) -> Instruments<'a> {
        Instruments { instruments }
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn get_instruments(&self) -> &Vec<&'a Instrument<'a>> {
        &self.instruments
    }

    pub fn get_underlying_codes(&self) -> Vec<&'a str> {
        let mut underlying_names = HashSet::<&str>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.as_trait().get_underlying_codes();
            for name in names.iter() {
                underlying_names.insert(name);
            }
        }
        underlying_names.into_iter().collect()
    }
    
    pub fn instruments_with_underlying(&self, und_code: &str) -> Vec<&'a Instrument<'a>> {
        let mut instruments = vec![];
        for instrument in self.instruments.iter() {
            let names = instrument.as_trait().get_underlying_codes();
            if names.contains(&und_code) {
                instruments.push(*instrument);
            }
        }
        instruments
    }
}