use crate::instrument::Instrument;
use crate::assets::currency::Currency;
use std::collections::HashMap;
use crate::enums::{CreditRating, IssuerType, RankType};

pub struct MatchPrameter<'a> {
    // (type_name: &'static str, currency: Currency) -> &'static str
    collateral_curve_map: HashMap<(&'static str, Currency), String>,

    // underlying_name: &'static str -> &'static str
    borrowing_curve_map: HashMap<(String, Currency), String>,
    
    // (issuer: String, 
    //  issuer_type: IssuerType, 
    //  credit_rating: CreditRating, 
    //  currency: Currency) -> String
    bond_discount_curve_map: HashMap<(
        String, 
        IssuerType, 
        CreditRating, 
        Currency
    ), String>,
}

impl Default for MatchPrameter {
    fn default() -> MatchPrameter {
        let collateral_curve_map: HashMap<(
            &'static str,
            Currency,
        ), String> = HashMap::new();

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

impl MatchPrameter {
    pub fn new(
        collateral_curve_map: HashMap<(
            String,
            Currency,
        ), String>,
        borrowing_curve_map: HashMap<(String, Currency), String>,
        bond_discount_curve_map: HashMap<(
            String, 
            IssuerType, 
            CreditRating, 
            Currency
        ), String>,
    ) -> MatchPrameter {
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
        }
    }

    pub fn get_discount_curve_name(&self, instrument: &Instrument) -> &String {
        match instrument {
            Instrument::FixedCouponBond(instrument) |
            Instrument::FloatingRateNote(instrument) => {
                self.bond_discount_curve_map.get(&(
                    instrument.get_issuer_name().unwrap(),
                    *instrument.get_issuer_type().unwrap(),
                    *instrument.get_credit_rating().unwrap(),
                    *instrument.get_currency(),
                )).expect("Bond has no discount curve").as_ref()
            },
            Instrument::IRS(instrument) => {
                instrument.get_rate_forward_curve_name()
                .expect("IRS has no rate forward curve").as_ref()
            },
            Instrument::StockFutures(_) |
            Instrument::BondFutures(_) |
            Instrument::KTBF(_) => "Dummy".to_String().as_ref(), // no discount
        }
    }

    pub fn get_bond_discount_curve(
        &self, 
        currency: &Currency,
        credit_rating: Option<&CreditRating>,
        issuer_type: Option<&IssuerType>,
        rank_type: Option<&RankType>,
    ) -> &String {
        "not yet implemented".to_string().as_ref()
    }

    pub fn get_collateral_curve_name(&self, instrument: &Instrument) -> &String {
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

    pub fn get_borrowing_curve_name(&self, instrument: &Instrument) -> &'static str {
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

