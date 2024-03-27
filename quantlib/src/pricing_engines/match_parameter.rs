use crate::instrument::{self, Instrument, InstrumentTrait};
use crate::assets::currency::Currency;
use crate::enums::{
    CreditRating, 
    IssuerType,
    RateIndexCode,
};
use crate::instruments::plain_swap::PlainSwapType;
//
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchParameter {
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
    rate_index_forward_curve_map: HashMap<RateIndexCode, String>,
    // Currency::XXX -> String::from("XXXCRS")
    // But if XXX == USD, then it is String::from("USDOIS")
    crs_curve_map: HashMap<Currency, String>,
    //
    dummy_string: String,
}

impl Default for MatchParameter {
    fn default() -> MatchParameter {
        let collateral_curve_map: HashMap<String, String> = HashMap::new();

        let borrowing_curve_map: HashMap<String, String> = HashMap::new();
        
        let bond_discount_curve_map: HashMap<(
            String,
            IssuerType, 
            CreditRating, 
            Currency
        ), String> = HashMap::new();

        let crs_curve_map: HashMap<Currency, String> = HashMap::new();
        
        let rate_index_forward_curve_map: HashMap<RateIndexCode, String> = HashMap::new();
        MatchParameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_forward_curve_map,
            crs_curve_map,
            dummy_string: String::from("Dummy"),
        }
    }
}

impl MatchParameter {
    pub fn new(
        collateral_curve_map: HashMap<String, String>,
        borrowing_curve_map: HashMap<String, String>, 
        bond_discount_curve_map: HashMap<(
            String, 
            IssuerType, 
            CreditRating, 
            Currency
        ), String>,
        crs_curve_map: HashMap<Currency, String>,
        rate_index_forward_curve_map: HashMap<RateIndexCode, String>
    ) -> MatchParameter {
        MatchParameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_forward_curve_map,
            crs_curve_map,
            dummy_string: String::from("Dummy"),
        }
    }

    pub fn get_crs_curve_name(&self, instrument: &Instrument) -> Result<&String> {
        match instrument {
            Instrument::PlainSwap(instrument) => {
                if instrument.get_specific_type() == PlainSwapType::IRS {
                    return Ok(&self.dummy_string);
                }

                let fixed_currency = instrument.get_fixed_leg_currency()?;
                let res = self.crs_curve_map.get(fixed_currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but its crs curve is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        fixed_currency.as_str(), 
                    ))?;                
                Ok(res)
            },
            Instrument::FxFutures(instrument) => {
                let currency = instrument.get_currency();
                let res = self.crs_curve_map.get(currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but its crs curve is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        currency.as_str()
                    ))?;
                Ok(res)
            },
            _ => Ok(&self.dummy_string),
        }
    }

    pub fn get_floating_crs_curve_name(&self, instrument: &Instrument) -> Result<&String> {
        match instrument {
            Instrument::PlainSwap(instrument) => {
                if instrument.get_specific_type() == PlainSwapType::IRS {
                    return Ok(&self.dummy_string);
                }
                
                let floating_currency = instrument.get_floating_leg_currency()?;
                let res = self.crs_curve_map.get(floating_currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but it is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        floating_currency.as_str()
                    ))?;
                Ok(res)
            },
            Instrument::FxFutures(instrument) => {
                let underlying_currency = instrument.get_underlying_currency()?;
                let res = self.crs_curve_map.get(underlying_currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but it is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        underlying_currency.as_str()
                    ))?;
                Ok(res)
            },
            _ => Ok(&self.dummy_string),
        }
    }
    pub fn get_discount_curve_name(&self, instrument: &Instrument) -> Result<&String> {
        match instrument {
            Instrument::Bond(instrument) => {
                match self.bond_discount_curve_map.get(&(
                    instrument.get_issuer_name().with_context(
                        || anyhow!(
                            "({}:{}) Issuer name is not found for {} ({})", 
                            file!(), line!(),
                            instrument.get_name(), instrument.get_code()))?.clone(),
                    instrument.get_issuer_type().with_context(
                        || anyhow!(
                            "({}:{}) Issuer type is not found for {} ({})", 
                            file!(), line!(),
                            instrument.get_name(), instrument.get_code()))?.clone(),
                    instrument.get_credit_rating().with_context(
                        || anyhow!(
                            "({}:{}) Credit rating is not found for {} ({})", 
                            file!(), line!(),
                            instrument.get_name(), instrument.get_code()))?.clone(),
                    instrument.get_currency().clone(),
                )) {
                    Some(curve_name) => Ok(curve_name),
                    None => Ok(&self.dummy_string),
                }
            }
            // IRS (or OIS) uses rate index forward curve as discount curve
            Instrument::PlainSwap(instrument) => {
                let rate_index = instrument.get_rate_index()
                    .context("Rate index is not found").unwrap();
                    
                match rate_index {
                    None => Ok(&self.dummy_string),
                    Some(rate_index) => {
                        match self.rate_index_forward_curve_map.get(rate_index.get_code()) {
                            Some(curve_name) => Ok(curve_name),
                            None => {
                                Err(anyhow!(
                                    "Rate index forward curve is not found for {}",
                                    rate_index.get_code()
                                ))
                            }
                        }
                    }
                }
            },
            // these are indestruments that do not need to be discounted
            Instrument::StockFutures(_) |
            Instrument::BondFutures(_) |
            Instrument::KTBF(_) |
            Instrument::FxFutures(_) => {
                Ok(&self.dummy_string)
            },
        }
    }
    /// Curve name for underlying asset
    /// This retrives the curve name from self.collateral_curve_map
    pub fn get_collateral_curve_names(&self, instrument: &Instrument) -> Result<Vec<&String>> {
        let und_codes = instrument.get_underlying_codes();
        let res = und_codes.iter().map(
            |code| {self.collateral_curve_map.get(*code)
                    .ok_or_else(|| anyhow!(
                        "{} has underlying code {} but no collateral curve name in MatchParameter.collateral_curve_map",
                        instrument.get_name(),
                        code))}).collect();
        res
    }

    pub fn get_collateral_curve_name(&self, instrument: &Instrument, und_code: &String) -> Result<&String> {
        self.collateral_curve_map.get(und_code)
        .ok_or_else(|| anyhow!(
            "{} has underlying code {} but no collateral curve name in MatchParameter.collateral_curve_map",
            instrument.get_name(),
            und_code))
    }
    /// Curve name for underlying asset
    /// This retrives the curve name from self.collateral_curve_map
    pub fn get_borrowing_curve_names(&self, instrument: &Instrument) -> Result<Vec<&String>> {
        let und_codes = instrument.get_underlying_codes();
        let res = und_codes.iter().map(|code| {
            self.borrowing_curve_map.get(*code)
            .with_context(|| anyhow!(
                "{} has underlying code {} but no borrowing curve name in MatchParameter.collateral_curve_map",
                instrument.get_name(),
                code))}).collect();
        res
    }

    pub fn get_rate_index_curve_name(&self, instrument: &Instrument) -> Result<&String> {
        match instrument {
            Instrument::Bond(instrument) => {
                let rate_index = instrument.get_rate_index()?;
                let res = match rate_index {
                    None => Ok(&self.dummy_string),
                    Some(rate_index) => {
                        self.rate_index_forward_curve_map.get(rate_index.get_code())
                        .ok_or_else(|| anyhow!(
                            "Rate index forward curve is not found for {}",
                            rate_index.get_code()))
                    }
                };
                res
            },
            Instrument::PlainSwap(instrument) => {
                let rate_index = instrument.get_rate_index()?;
                let res = match rate_index {
                    None => Ok(&self.dummy_string),
                    Some(rate_index) => {
                        self.rate_index_forward_curve_map.get(rate_index.get_code())
                        .ok_or_else(|| anyhow!(
                            "Rate index forward curve is not found for {}",
                            rate_index.get_code()))
                    }
                };
                res
            },
            _ => Ok(&self.dummy_string),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::instruments::stock_futures::StockFutures;
    use crate::instruments::plain_swap::PlainSwap;
    use crate::assets::currency::Currency;
    use crate::enums::{RateIndexCode, CreditRating, IssuerType};
    use std::collections::HashMap;
    use time::convert::Day;
    use time::macros::datetime;
    use crate::time::conventions::{BusinessDayConvention, PaymentFrequency, DayCountConvention};
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::time::jointcalendar::JointCalendar;
    use crate::time::calendar::Calendar;
    use crate::parameters::rate_index::RateIndex;
    use time::Duration;
    use anyhow::Result;

    #[test]
    fn test_match_parameter() -> Result<()> {
        let mut collateral_curve_map: HashMap<String, String> = HashMap::new();
        let borrowing_curve_map: HashMap<String, String> = HashMap::new();
        let bond_discount_curve_map: HashMap<(
            String, 
            IssuerType, 
            CreditRating, 
            Currency
        ), String> = HashMap::new();
        let mut rate_index_forward_curve_map: HashMap<RateIndexCode, String> = HashMap::new();

        let stock_futures = StockFutures::new(
            100.0,
            datetime!(2021-01-01 00:00:00 +00:00),
            datetime!(2021-12-31 00:00:00 +00:00),
            datetime!(2021-12-31 00:00:00 +00:00),
            datetime!(2021-12-31 00:00:00 +00:00),
            100.0,
            Currency::USD,
            Currency::USD,
            "AAPL".to_string(),
            "AAPL".to_string(),
            "AAPL".to_string(),
        );

        // let's make SouthKorea - Setlement calendar 
        // By the reason of project architecture, its is inherently JointCalendar

        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let calendar = Calendar::SouthKorea(sk);
        let joint_calendar = JointCalendar::new(vec![calendar])?;

        // make a CD 3M RateIndex
        let cd = RateIndex::new(
            String::from("91D"),
            Currency::KRW,
            RateIndexCode::CD,
            "CD91".to_string(),
        )?;

        let issue_date = datetime!(2021-01-01 00:00:00 +00:00);
        let maturity_date = datetime!(2021-12-31 00:00:00 +00:00);
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
            Some(cd.clone()),
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
            "IRS".to_string(),
            "IRS".to_string(),
        )?;
        
        collateral_curve_map.insert("AAPL".to_string(), String::from("USDGOV"));
        rate_index_forward_curve_map.insert(RateIndexCode::CD, "KRWIRS".to_string());

        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            HashMap::new(),
            rate_index_forward_curve_map,
        );
        let stock_futures_inst = Instrument::StockFutures(stock_futures);
        let irs_inst = Instrument::PlainSwap(irs);

        assert_eq!(
            match_parameter.get_collateral_curve_name(
                &stock_futures_inst,
                &String::from("AAPL")
            )?.clone(),
            String::from("USDGOV"),
            "StockFutures has underlying code AAPL but it returns a curve name: {}",
            match_parameter.get_collateral_curve_name(
                &stock_futures_inst,
                &String::from("AAPL")
            )?
        );

        assert_eq!(
            match_parameter.get_discount_curve_name(&stock_futures_inst)?.clone(), 
            String::from("Dummy"),
            "StockFutures does not need to be discounted but it returns a curve name: {}",
            match_parameter.get_discount_curve_name(&stock_futures_inst)?
        );

        assert_eq!(
            match_parameter.get_rate_index_curve_name(&stock_futures_inst)?.clone(), 
            String::from("Dummy"),
            "StockFutures does not need to be discounted but it returns a curve name: {}",
            match_parameter.get_rate_index_curve_name(&stock_futures_inst)?
        );

        assert_eq!(
            match_parameter.get_discount_curve_name(&irs_inst)?.clone(), 
            String::from("KRWIRS"),
            "IRS needs to be discounted but it returns a curve name: {}",
            match_parameter.get_discount_curve_name(&irs_inst)?
        );

        assert_eq!(
            match_parameter.get_rate_index_curve_name(&irs_inst)?.clone(), 
            String::from("KRWIRS"),
            "IRS needs to be discounted but it returns a curve name: {}",
            match_parameter.get_rate_index_curve_name(&irs_inst)?
        );
        Ok(())
    }
}

