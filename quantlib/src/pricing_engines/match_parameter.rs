use crate::instrument::Instrument;
use crate::assets::currency::Currency;
use std::collections::HashMap;
use crate::enums::{CreditRating, IssuerType, RankType};

pub struct MatchPrameter<'a> {
    // (type_name: &'static str, currency: Currency) -> &'static str
    collateral_curve_map: HashMap<(&'a str, Currency), &'a str>,

    // underlying_name: &'static str -> &'static str
    borrowing_curve_map: HashMap<(&'a str, Currency), &'a str>,
    
    // (issuer: &'a str, 
    //  issuer_type: IssuerType, 
    //  credit_rating: CreditRating, 
    //  currency: Currency) -> &'a str
    bond_discount_curve_map: HashMap<(
        &'a str, 
        IssuerType, 
        CreditRating, 
        Currency
    ), &'a str>,
}

impl<'a> Default for MatchPrameter<'a> {
    fn default() -> MatchPrameter<'a> {
        let collateral_curve_map: HashMap<(
            &str,
            Currency,
        ), &str> = HashMap::new();

        let borrowing_curve_map: HashMap<(&str, Currency), &str> = HashMap::new();
        
        let bond_discount_curve_map: HashMap<(
            &str, 
            IssuerType, 
            CreditRating, 
            Currency
        ), &str> = HashMap::new();
        
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
        }
    }
}

impl<'a> MatchPrameter<'a> {
    pub fn new(
        collateral_curve_map: HashMap<(
            &'a str,
            Currency,
        ), &'a str>,
        borrowing_curve_map: HashMap<(&'a str, Currency), &'a str>,
        bond_discount_curve_map: HashMap<(
            &'a str, 
            IssuerType, 
            CreditRating, 
            Currency
        ), &'a str>,
    ) -> MatchPrameter<'a> {
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
        }
    }

    pub fn get_discount_curve_name(&self, instrument: &Instrument<'a>) -> &'a str {
        match instrument {
            Instrument::FixedCouponBond(instrument) |
            Instrument::FloatingRateNote(instrument) => {
                self.bond_discount_curve_map.get(&(
                    instrument.get_issuer_name().unwrap(),
                    *instrument.get_issuer_type().unwrap(),
                    *instrument.get_credit_rating().unwrap(),
                    *instrument.get_currency(),
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
    ) -> &'a str {
        "not yet implemented"
    }

    pub fn get_collateral_curve_name(&self, instrument: &Instrument<'a>) -> &'a str {
        match instrument {
            Instrument::StockFutures(instrument) |
            Instrument::BondFutures(instrument) |
            Instrument::KTBF(instrument) => {
                self.collateral_curve_map.get(&(
                    instrument.get_type_name(),
                    *instrument.get_currency(),
                )).expect("Collateral curve is not found")
            },
            Instrument::FixedCouponBond(_) |
            Instrument::FloatingRateNote(_) |
            Instrument::IRS(_)=> {
                "collateral curve is not needed for IRS and Bonds"
            },
        }
    }

    pub fn get_borrowing_curve_name(&self, instrument: &Instrument<'a>) -> &'static str {
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

