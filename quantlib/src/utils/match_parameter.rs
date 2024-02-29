use crate::instrument::Instrument;
use crate::assets::currency::Currency;
use std::{collections::HashMap, hash::Hash};
use crate::enums::{CreditRating, IssuerType, RankType};

pub struct MatchPrameter {
    // (type_name: &'static str, currency: Currency) -> &'static str
    collateral_curve_map: HashMap<(&'static str, Currency), &'static str>,

    // underlying_name: &'static str -> &'static str
    borrowing_curve_map: HashMap<&'static str, &'static str>,
    
    // (issuer: &'static str, 
    //  credit_rating: CreditRating, 
    //  issuer_type: IssuerType, 
    //  currency: Currency) -> &'static str
    bond_discount_curve_map: HashMap<(
        &'static str, 
        CreditRating, 
        IssuerType, 
        Currency
    ), &'static str>,
}

impl Default for MatchPrameter {
    fn default() -> MatchPrameter {
        let collateral_curve_map: HashMap<(
            &str,
            Currency,
        ), &str> = HashMap::new();

        let borrowing_curve_map: HashMap<&str, &str> = HashMap::new();
        
        let bond_discount_curve_map: HashMap<(
            &str, 
            CreditRating, 
            IssuerType, 
            Currency
        ), &str> = HashMap::new();
        
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
        }
    }
}

impl MatchPrameter {
    pub fn new(
        collateral_curve_map: HashMap<(
            &str,
            Currency,
        ), &str>,
        borrowing_curve_map: HashMap<&str, &str>,
        bond_discount_curve_map: HashMap<(
            &str, 
            CreditRating, 
            IssuerType, 
            Currency
        ), &str>,
    ) -> MatchPrameter {
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
        }
    }

    pub fn get_discount_curve_name(&self, instrument: &Instrument) -> &'static str {
        match instrument {
            Instrument::FixedCouponBond(instrument) |
            Instrument::FloatingRateNote(instrument) => {
                self.bond_discount_curve_map.get(&(
                    instrument.get_currency(),
                    instrument.get_credit_rating(),
                    instrument.get_issuer_type(),
                    instrument.get_rank_type(),
                )).expect("Bond has no discount curve")
            },
            Instrument::IRS(instrument) => {
                instrument.get_rate_forward_curve_name()
                .expect("IRS has no rate forward curve")
            },
            Instrument::StockFutures(_) |
            Instrument::BondFutures(_) |
            Instrument::KTBF(_) => "Dummy", // no discount
        }
    }

    pub fn get_bond_discount_curve(
        &self, 
        currency: &Currency,
        credit_rating: Option<&CreditRating>,
        issuer_type: Option<&IssuerType>,
        rank_type: Option<&RankType>,
    ) -> &'static str {
        "not yet implemented"
    }

    pub fn get_collateral_curve_name(&self, instrument: &Instrument) -> &'static str {
        match instrument {
            Instrument::StockFutures(instrument) |
            Instrument::BondFutures(instrument) |
            Instrument::KTBF(instrument) => {
                instrument.get_currency().as_str()
            },
            Instrument::FixedCouponBond(_) |
            Instrument::FloatingRateNote(_) |
            Instrument::IRS(_)=> {
                "collateral curve is not needed for IRS and Bonds"
            },
        }
    }

    pub fn get_repo_curve_name(&self, instrument: &Instrument) -> &'static str {
        match instrument {
            Instrument::StockFutures(instrument) => {
                match instrument.get_currency() {
                    Currency::KRW => "KRW",
                    _ => "Undefined",
                }
            }
            _ => "Undefined",
        }
    }
}

