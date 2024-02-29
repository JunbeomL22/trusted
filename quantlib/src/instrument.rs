use crate::definitions::Real;
use crate::assets::currency::Currency;
use time::OffsetDateTime;
use std::ops::Index;
use std::collections::HashSet;
use crate::enums::{IssuerType, CreditRating, RankType};

pub trait InstrumentTriat<'a> {
    // The following methods are mandatory for all instruments
    fn get_name(&self) -> &'a str;
    fn get_code(&self) -> &'a str;
    fn get_currency(&self) -> &Currency;
    fn get_unit_notional(&self) -> Real;
    fn get_type_name(&self) -> &'a str;

    // There may be an instrument that does not need discount curve, e.g., Futures
    fn get_discount_curve_name(&self) -> &'static str { "Dummy" }

    // There is an instrument that does not have maturity date, so it is optional
    fn get_maturity(&self) -> Option<&OffsetDateTime> { None }

    // There is an instrument that does not have underlying names, 
    // so the default action is to return an empty vector
    fn get_underlying_names(&self) -> Vec<&'a str> { vec![] }

    // only for bonds, so None must be allowed
    fn get_credit_rating(&self) -> Option<&CreditRating> { None }
    // only for bonds, so None must be allowed
    fn get_issuer_type(&self) -> Option<&IssuerType> { None }
    // only for bonds, so None must be allowed
    fn get_rank_type(&self) -> Option<&RankType> { None }
    // only for bonds, so None must be allowed
    fn get_issuer_name(&self) -> Option<&'static str> { None }
    // only for instruments for floating coupon,
    // e.g., frn, irs, etc, so None must be allowed
    fn get_rate_forward_curve_name(&self) -> Option<&'static str> { None }
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
pub struct Instruments<'a> {
    instruments: Vec<&'a Instrument>,
}

impl<'a> Index<usize> for Instruments<'a> {
    type Output = Instrument;

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
    pub fn new(instruments: Vec<&'a Instrument>) -> Self {
        Instruments { instruments }
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn get_instruments(&self) -> &Vec<&'a Instrument> {
        &self.instruments
    }

    pub fn get_underlying_names(&self) -> Vec<String> {
        let mut underlying_names = HashSet::<String>::new();
        for instrument in self.instruments.iter() {
            if let Some(names) = instrument.as_trait().get_underlying_names() {
                for name in names.iter() {
                    underlying_names.insert(name.clone());
                }
            }
        }

        underlying_names.into_iter().collect()
    }

    pub fn select_instruments(
        &self, 
        types: &str,
        currency: Option<&Currency>,
        underlying_names: Option<&Vec<String>>,
        credit_rating: Option<&CreditRating>, // only used for bonds so None must be allowed
        issuer_type: Option<&IssuerType>, // only used for bonds so None must be allowed
        issuer_name: Option<&String>, // only used for bonds so None must be allowed
    ) -> Instruments {
        let mut selected_instruments = Vec::<&Instrument>::new();
        for instrument in self.instruments.iter() {
            if instrument.as_trait().get_type_name() == types {
                if let Some(currency) = currency {
                    if instrument.as_trait().get_currency() != currency {
                        continue;
                    }
                }
                if let Some(underlying_names) = underlying_names {
                    let mut lhs = instrument.as_trait().get_underlying_names().unwrap().clone();
                    lhs.sort();
                    let mut rhs = underlying_names.clone();
                    rhs.sort();
                    if lhs != rhs {
                        continue;
                    }
                }
                
                match instrument {
                    Instrument::FixedCouponBond(instrument) | 
                    Instrument::FloatingRateNote(instrument) => 
                    {
                        if let Some(credit_rating) = credit_rating {
                            if instrument.get_credit_rating().unwrap() != credit_rating {
                                continue;
                            }
                        }
                        if let Some(issuer_type) = issuer_type {
                            if instrument.get_issuer_type().unwrap() != issuer_type {
                                continue;
                            }
                        }
                        if let Some(issuer_name) = issuer_name {
                            if instrument.get_issuer_name().unwrap() != issuer_name {
                                continue;
                            }
                        }
                    }
                    _ => {}
                }

                selected_instruments.push(*instrument);
            }
        }
        Instruments::new(selected_instruments)
    }
}