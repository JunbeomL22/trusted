use crate::instrument;
use crate::{enums::RateIndexCode, instrument::Instrument};
use crate::assets::currency::Currency;
use std::collections::HashMap;
use crate::enums::{CreditRating, IssuerType, RankType};
use serde::{Serialize, Deserialize};
use crate::parameters::rate_index::RateIndex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchPrameter {
    // Underlying asset code: String -> curve_name: String
    // Underlying code examples are stock, bond, commodity, etc.
    collateral_curve_map: HashMap<String, String>,

    // Underlying asset code: String -> curve_name: String
    // Underlying code examples are stock, bond, commodity, etc.
    borrowing_curve_map: HashMap<String, String>,
    
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

    // index code: RateIndexCode -> String
    rate_index_forward_curve_map: HashMap<RateIndexCode, String>
}

impl Default for MatchPrameter {
    fn default() -> MatchPrameter {
        let collateral_curve_map: HashMap<String, String> = HashMap::new();

        let borrowing_curve_map: HashMap<String, String> = HashMap::new();
        
        let bond_discount_curve_map: HashMap<(
            String,
            IssuerType, 
            CreditRating, 
            Currency
        ), String> = HashMap::new();
        
        let rate_index_forward_curve_map: HashMap<RateIndexCode, String> = HashMap::new();
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_forward_curve_map,
        }
    }
}

impl MatchPrameter {
    pub fn new(
        collateral_curve_map: HashMap<String, String>,
        borrowing_curve_map: HashMap<String, String>, 
        bond_discount_curve_map: HashMap<(
            String, 
            IssuerType, 
            CreditRating, 
            Currency
        ), String>,
        rate_index_forward_curve_map: HashMap<RateIndexCode, String>
    ) -> MatchPrameter {
        MatchPrameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_forward_curve_map,
        }
    }

    pub fn get_discount_curve_name(&self, instrument: &Instrument) -> &String {
        match instrument {
            Instrument::FixedCouponBond(instrument) |
            Instrument::FloatingRateNote(instrument) => {
                match self.bond_discount_curve_map.get(&(
                    *instrument.get_issuer_name().expect("Issuer name is not found"),
                    *instrument.get_issuer_type().expect("Issuer type is not found"),
                    *instrument.get_credit_rating().expect("Credit rating is not found"),
                    *instrument.get_currency(),
                )) {
                    Some(curve_name) => curve_name,
                    None => &"Dummy".to_string(), 
                }
            },
            // IRS (or OIS) uses rate index forward curve as discount curve
            Instrument::IRS(instrument) => {
                let code = instrument.get_rate_index()
                    .expect("Rate index is not found")
                    .get_code();
                match self.rate_index_forward_curve_map.get(code) {
                    Some(curve_name) => curve_name,
                    None => &"Dummy".to_string(), 
                }
            },
            // these are indestruments that do not need to be discounted
            Instrument::StockFutures(_) |
            Instrument::BondFutures(_) |
            Instrument::KTBF(_) => &"Dummy".to_string(), // no discount
        }
    }

    /// Curve name for underlying asset
    /// This retrives the curve name from self.collateral_curve_map
    pub fn get_collateral_curve_names(&self, instrument: &Instrument) -> &Vec<&String> {
        let und_codes = instrument.as_trait().get_underlying_codes();
        let res = und_codes.iter().map(|code| {
            self.collateral_curve_map.get(code)
            .expect(format!(
                "{} has underlying code {} but no collateral curve name in MatchParameter.collateral_curve_map",
                instrument.as_trait().get_name(),
                code
            ).as_str())}).collect();
        &res
    }

    /// Curve name for underlying asset
    /// This retrives the curve name from self.collateral_curve_map
    pub fn get_borrowing_curve_names(&self, instrument: &Instrument) -> &Vec<&String> {
        let und_codes = instrument.as_trait().get_underlying_codes();
        let res = und_codes.iter().map(|code| {
            self.borrowing_curve_map.get(code)
            .expect(format!(
                "{} has underlying code {} but no borrowing curve name in MatchParameter.collateral_curve_map",
                instrument.as_trait().get_name(),
                code
            ).as_str())}).collect();
        &res
    }

    pub fn get_rate_index_curve_name(&self, instrument: &Instrument) -> &String {
        match instrument {
            Instrument::IRS(instrument) |
            Instrument::FloatingRateNote(instrument) => {
                self.rate_index_forward_curve_map.get(
                    instrument.get_rate_index()
                    .expect("Rate index is not found")
                    .get_code()
                ).expect("Rate index forward curve is not found")
            },
            _ => &"Dummy".to_string(),
        }
    }
}

