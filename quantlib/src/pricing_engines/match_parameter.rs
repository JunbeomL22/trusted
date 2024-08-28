use crate::currency::Currency;
use crate::enums::{
    CreditRating,
    IssuerType,
    //RateIndexCode,
    OptionDailySettlementType,
};
use crate::instrument::{Instrument, InstrumentTrait};
use crate::instruments::plain_swap::PlainSwapType;
use crate::ID;
//
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;
//
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchParameter {
    // Underlying asset code: ID -> curve_id: ID
    // Underlying code examples are stock, bond, commodity, etc.
    collateral_curve_map: FxHashMap<ID, ID>,
    // Underlying asset code: ID -> curve_id: ID
    // Underlying code examples are stock, bond, commodity, etc.
    borrowing_curve_map: FxHashMap<ID, ID>,
    // (issuer: ID,
    //  issuer_type: IssuerType,
    //  credit_rating: CreditRating,
    //  currency: Currency) -> ID
    bond_discount_curve_map: FxHashMap<(ID, IssuerType, CreditRating, Currency), ID>,
    // index code: RateIndexCode -> ID
    rate_index_forward_curve_map: FxHashMap<ID, ID>,
    // Currency::XXX -> ID::from("XXXCRS")
    // But if XXX == USD, then it is ID::from("USDOIS")
    crs_curve_map: FxHashMap<Currency, ID>,
    //
    funding_cost_map: FxHashMap<Currency, ID>,
    //
}

impl Default for MatchParameter {
    fn default() -> MatchParameter {
        let collateral_curve_map: FxHashMap<ID, ID> = FxHashMap::default();

        let borrowing_curve_map: FxHashMap<ID, ID> = FxHashMap::default();

        let bond_discount_curve_map: FxHashMap<(ID, IssuerType, CreditRating, Currency), ID> = FxHashMap::default();

        let crs_curve_map: FxHashMap<Currency, ID> = FxHashMap::default();
        let funding_cost_map: FxHashMap<Currency, ID> = FxHashMap::default();
        let rate_index_forward_curve_map: FxHashMap<ID, ID> = FxHashMap::default();
        
        MatchParameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_forward_curve_map,
            crs_curve_map,
            funding_cost_map,
        }
    }
}

impl MatchParameter {
    pub fn new(
        collateral_curve_map: FxHashMap<ID, ID>,
        borrowing_curve_map: FxHashMap<ID, ID>,
        bond_discount_curve_map: FxHashMap<(ID, IssuerType, CreditRating, Currency), ID>,
        crs_curve_map: FxHashMap<Currency, ID>,
        rate_index_forward_curve_map: FxHashMap<ID, ID>,
        funding_cost_map: FxHashMap<Currency, ID>,
    ) -> MatchParameter {
        MatchParameter {
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            rate_index_forward_curve_map,
            crs_curve_map,
            funding_cost_map,
        }
    }

    /// In the cases of crs, fx products, etc, this means the base_curve
    /// For example, if the undrlying fx is usdkrw, then crs_curve is krwcrs
    pub fn get_crs_curve_id(&self, instrument: &Instrument) -> Result<ID> {
        match instrument {
            Instrument::PlainSwap(instrument) => {
                if instrument.get_specific_plain_swap_type()? == PlainSwapType::IRS {
                    return Ok(ID::default());
                }

                let fixed_currency = instrument.get_fixed_leg_currency()?;
                let res = self.crs_curve_map.get(&fixed_currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but its crs curve is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        fixed_currency.as_str(),
                    ))?;
                Ok(*res)
            }
            Instrument::FxFutures(instrument) => {
                let currency = instrument.get_currency();
                let res = self.crs_curve_map.get(&currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but its crs curve is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        currency.as_str()
                    ))?;
                Ok(*res)
            }
            _ => Ok(ID::default()),
        }
    }

    pub fn get_floating_crs_curve_id(&self, instrument: &Instrument) -> Result<ID> {
        match instrument {
            Instrument::PlainSwap(instrument) => {
                if instrument.get_specific_plain_swap_type()? == PlainSwapType::IRS {
                    return Ok(ID::default());
                }

                let floating_currency = instrument.get_floating_leg_currency()?;
                let res = self.crs_curve_map.get(&floating_currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but it is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        floating_currency.as_str()
                    ))?;
                Ok(*res)
            }
            Instrument::FxFutures(instrument) => {
                let underlying_currency = instrument.get_underlying_currency()?;
                let res = self.crs_curve_map.get(&underlying_currency)
                    .ok_or_else(|| anyhow!(
                        "({}:{}) {} ({}) has {}, but it is not found in MatchParameter.crs_curve_map",
                        file!(), line!(),
                        instrument.get_name(), instrument.get_code(),
                        underlying_currency.as_str()
                    ))?;
                Ok(*res)
            }
            _ => Ok(ID::default()),
        }
    }
    pub fn get_discount_curve_id(&self, instrument: &Instrument) -> Result<ID> {
        let id = instrument.get_id();
        let base_msg = format!("discount curve not found for ({:?})", id);
        match instrument {
            Instrument::Bond(instrument) => {
                let id = instrument.get_issuer_id().with_context(|| anyhow!(base_msg.clone()))?;
                let issuer_type = instrument.get_issuer_type().with_context(|| anyhow!(base_msg.clone()))?;
                let credit_rating = instrument.get_credit_rating().with_context(|| anyhow!(base_msg.clone()))?;
                match self.bond_discount_curve_map.get(&(
                    id,
                    issuer_type,
                    credit_rating,
                    instrument.get_currency(),
                )) {
                    Some(curve_id) => Ok(*curve_id),
                    None => Ok(ID::default()),
                }
            }
            // IRS (or OIS) uses rate index forward curve as discount curve
            Instrument::PlainSwap(instrument) => {
                let rate_index = instrument
                    .get_rate_index()
                    .context("Rate index is not found")
                    .unwrap();

                match rate_index {
                    None => Ok(ID::default()),
                    Some(rate_index) => {
                        match self.rate_index_forward_curve_map.get(&rate_index.get_id()) {
                            Some(curve_id) => Ok(*curve_id),
                            None => Err(anyhow!(
                                "Rate index forward curve is not found for {:?}",
                                rate_index.get_rate_index_symbol_str(),
                            )),
                        }
                    }
                }
            }
            Instrument::VanillaOption(instrument) => {
                match instrument.get_option_daily_settlement_type()? {
                    OptionDailySettlementType::Settled => Ok(ID::default()),
                    OptionDailySettlementType::NotSettled => {
                        match self.funding_cost_map.get(&instrument.get_currency()) {
                            Some(curve_id) => Ok(*curve_id),
                            None => {
                                Err(anyhow!(
                                    "({}:{}) Risk free rate curve is not found for {} ({}).\n\
                                    The Option's currency is {:?} but its curve is not found in MatchParameter.funding_cost",
                                    file!(), line!(), instrument.get_name(), instrument.get_code(), instrument.get_currency(),
                                ))
                            }
                        }
                    }
                }
            }
            // these are indestruments that do not need to be discounted
            Instrument::Futures(_)
            | Instrument::BondFutures(_)
            | Instrument::KTBF(_)
            | Instrument::FxFutures(_)
            | Instrument::Stock(_)
            | Instrument::Cash(_) => Ok(ID::default()),
        }
    }
    /// Curve name for underlying asset
    /// This retrives the curve name from self.collateral_curve_map
    pub fn get_collateral_curve_ids(&self, instrument: &Instrument) -> Result<Vec<ID>> {
        let und_ids = instrument.get_underlying_ids();
        let mut res = vec![];
        for id in und_ids {
            match self.collateral_curve_map.get(&id) {
                Some(curve_id) => res.push(*curve_id),
                None => {
                    let err = || anyhow!(
                        "{} has underlying ({:?}) but no collateral curve name in MatchParameter.collateral_curve_map",
                        instrument.get_name(),
                        id
                    );
                    return Err(err());
                }
            }
        }
        Ok(res)
    }

    pub fn get_collateral_curve_id(
        &self,
        instrument: &Instrument,
        und_id: ID,
    ) -> Result<ID> {
        if let Some(id) = self.collateral_curve_map.get(&und_id) {
            return Ok(*id);
        } else {
            let err = || anyhow!(
                "{} has underlying ({:?}) but no collateral curve name in MatchParameter.collateral_curve_map",
                instrument.get_name(),
                und_id
            );
            return Err(err());
        }
    }
    /// Curve name for underlying asset
    /// This retrives the curve name from self.collateral_curve_map
    pub fn get_borrowing_curve_ids(&self, instrument: &Instrument) -> Result<Vec<ID>> {
        let mut und_ids = instrument.get_underlying_ids();
        let bond_futures_collateral_ids = instrument.get_bond_futures_borrowing_curve_ids();
        if !bond_futures_collateral_ids.is_empty() {
            und_ids.append(&mut bond_futures_collateral_ids.clone());
        }

        let mut res = vec![];
        for id in und_ids {
            match self.borrowing_curve_map.get(&id) {
                Some(curve_id) => res.push(*curve_id),
                None => {
                    let err = || anyhow!(
                        "{} has underlying ({:?}) but no borrowing curve name in MatchParameter.collateral_curve_map",
                        instrument.get_name(),
                        id
                    );
                    return Err(err());
                }
            }
        }

        Ok(res)
    }

    pub fn get_rate_index_curve_id(&self, instrument: &Instrument) -> Result<ID> {
        match instrument {
            Instrument::Bond(instrument) => {
                let rate_index = instrument.get_rate_index()?;
                match rate_index {
                    None => Ok(ID::default()),
                    Some(rate_index) => {
                        let res = self.rate_index_forward_curve_map.get(&rate_index.get_id())
                        .ok_or_else(|| anyhow!(
                            "Rate index forward curve is not found for {:?}",
                            rate_index.get_id()
                        ))?;
                        Ok(*res)
                    }
                }
            },
            Instrument::PlainSwap(instrument) => {
                let rate_index = instrument.get_rate_index()?;
                match rate_index {
                    None => Ok(ID::default()),
                    Some(rate_index) => {
                        let res = self.rate_index_forward_curve_map.get(rate_index.get_code())
                        .ok_or_else(|| anyhow!(
                            "Rate index forward curve is not found for {:?}",
                            rate_index.get_id()
                        ))?;
                        Ok(*res)
                    }
                }
            },
            _ => Ok(ID::default()),
        }
    }

    pub fn get_collateral_curve_map(&self) -> &FxHashMap<ID, ID> {
        &self.collateral_curve_map
    }

    pub fn get_borrowing_curve_map(&self) -> &FxHashMap<ID, ID> {
        &self.borrowing_curve_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::currency::Currency;
    use crate::enums::{CreditRating, IssuerType};
    use crate::instruments::futures::Futures;
    use crate::instruments::plain_swap::PlainSwap;
    use crate::parameters::rate_index::RateIndex;
    use crate::time::calendar::Calendar;
    use crate::time::calendars::southkorea::{SouthKorea, SouthKoreaType};
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use crate::time::jointcalendar::JointCalendar;
    use anyhow::Result;
    use std::collections::HashMap;
    use time::macros::datetime;

    #[test]
    fn test_match_parameter() -> Result<()> {
        let mut collateral_curve_map: HashMap<String, String> = HashMap::new();
        let borrowing_curve_map: HashMap<String, String> = HashMap::new();
        let bond_discount_curve_map: HashMap<(String, IssuerType, CreditRating, Currency), String> =
            HashMap::new();
        let mut rate_index_forward_curve_map: HashMap<String, String> = HashMap::new();

        let stock_futures = Futures::new(
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
            "CD 91D".to_string(),
            "CD 91D".to_string(),
        )?;

        let issue_date = datetime!(2021-01-01 00:00:00 +00:00);
        let maturity_date = datetime!(2021-12-31 00:00:00 +00:00);
        let irs = PlainSwap::new_from_conventions(
            Currency::KRW,
            Currency::KRW,
            //
            None,
            None,
            None,
            None,
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
        rate_index_forward_curve_map.insert("CD 91D".to_string(), "KRWIRS".to_string());

        let funding_cost_map: HashMap<Currency, String> = HashMap::new();
        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_discount_curve_map,
            HashMap::new(),
            rate_index_forward_curve_map,
            funding_cost_map,
        );

        let stock_futures_inst = Instrument::Futures(stock_futures);
        let irs_inst = Instrument::PlainSwap(irs);

        assert_eq!(
            match_parameter
                .get_collateral_curve_id(&stock_futures_inst, &String::from("AAPL"))?
                .clone(),
            String::from("USDGOV"),
            "EquityFutures has underlying code AAPL but it returns a curve name: {}",
            match_parameter
                .get_collateral_curve_id(&stock_futures_inst, &String::from("AAPL"))?
        );

        assert_eq!(
            match_parameter
                .get_discount_curve_id(&stock_futures_inst)?
                .clone(),
            String::from("Dummy"),
            "EquityFutures does not need to be discounted but it returns a curve name: {}",
            match_parameter.get_discount_curve_id(&stock_futures_inst)?
        );

        assert_eq!(
            match_parameter
                .get_rate_index_curve_id(&stock_futures_inst)?
                .clone(),
            String::from("Dummy"),
            "EquityFutures does not need to be discounted but it returns a curve name: {}",
            match_parameter.get_rate_index_curve_id(&stock_futures_inst)?
        );

        assert_eq!(
            match_parameter.get_discount_curve_id(&irs_inst)?.clone(),
            String::from("KRWIRS"),
            "IRS needs to be discounted but it returns a curve name: {}",
            match_parameter.get_discount_curve_id(&irs_inst)?
        );

        assert_eq!(
            match_parameter
                .get_rate_index_curve_id(&irs_inst)?
                .clone(),
            String::from("KRWIRS"),
            "IRS needs to be discounted but it returns a curve name: {}",
            match_parameter.get_rate_index_curve_id(&irs_inst)?
        );
        Ok(())
    }
}
