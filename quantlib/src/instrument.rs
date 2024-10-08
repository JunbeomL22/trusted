use crate::currency::{Currency, FxCode};
use crate::definitions::Real;
use crate::enums::{
    AccountingLevel, CreditRating, IssuerType, OptionDailySettlementType, OptionType, RankType,
};

use crate::instruments::schedule::Schedule;
use crate::instruments::{
    bond::Bond,
    bond_futures::BondFutures,
    cash::Cash,
    futures::Futures,
    fx_futures::FxFutures,
    ktbf::KTBF,
    plain_swap::{PlainSwap, PlainSwapType},
    stock::Stock,
    vanilla_option::VanillaOption,
};

use crate::parameters::{
    past_price::DailyClosePrice, rate_index::RateIndex, zero_curve::ZeroCurve,
};
use crate::pricing_engines::match_parameter::MatchParameter;
use crate::time::{conventions::PaymentFrequency, jointcalendar::JointCalendar};
//
use anyhow::{anyhow, Context, Result};
use enum_dispatch::enum_dispatch;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Index,
    rc::Rc,
};
use time::OffsetDateTime;

#[enum_dispatch]
pub trait InstrumentTrait {
    // The following methods are mandatory for all instruments
    fn get_name(&self) -> &String;
    fn get_code(&self) -> &String;
    fn get_currency(&self) -> &Currency;
    fn get_unit_notional(&self) -> Real;
    fn get_type_name(&self) -> &'static str;
    fn get_average_trade_price(&self) -> Real {
        0.0
    }
    //
    fn get_accountring_level(&self) -> AccountingLevel {
        AccountingLevel::Level1
    }
    //
    // There is an instrument that does not have maturity date, so it is optional
    fn get_maturity(&self) -> Option<&OffsetDateTime> {
        None
    }
    // There is an instrument that does not have underlying names,
    // so the default action is to return an empty vector
    fn get_underlying_codes(&self) -> Vec<&String> {
        vec![]
    }

    fn get_quanto_fxcode_und_pair(&self) -> Vec<(&String, &FxCode)> {
        vec![]
    }

    fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> {
        vec![]
    }

    fn get_underlying_codes_requiring_volatility(&self) -> Vec<&String> {
        vec![]
    }
    // only for bonds, so None must be allowed
    fn get_credit_rating(&self) -> Result<&CreditRating> {
        Err(anyhow!(
            "({}:{}) not supported instrument type on get_credit_rating",
            file!(),
            line!()
        ))
    }
    // only for bonds, so None must be allowed
    fn get_issuer_type(&self) -> Result<&IssuerType> {
        Err(anyhow!(
            "({}:{}) not supported instrument type on get_issuer_type",
            file!(),
            line!()
        ))
    }
    // only for bonds, so None must be allowed
    fn get_rank_type(&self) -> Result<&RankType> {
        Err(anyhow!(
            "({}:{}) not supported instrument type on get_rank_type",
            file!(),
            line!()
        ))
    }
    // only for bonds, so None must be allowed
    fn get_issuer_name(&self) -> Result<&String> {
        Err(anyhow!(
            "({}:{}) not supported instrument type on get_issuer_name",
            file!(),
            line!()
        ))
    }

    // only for FloatingRateNote, IRS, OIS, and other swaps
    fn get_rate_index(&self) -> Result<Option<&RateIndex>> {
        Err(anyhow!(
            "({}:{}) not supported instrument type on get_rate_index",
            file!(),
            line!()
        ))
    }

    fn get_bond_futures_borrowing_curve_tags(&self) -> Vec<&String> {
        vec![]
    }

    fn get_cashflows(
        &self,
        _pricing_date: &OffsetDateTime,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        _past_data: Option<Rc<DailyClosePrice>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        Err(anyhow!(
            "not supported instrument type on get_coupon_cashflow"
        ))
    }

    fn get_floating_cashflows(
        &self,
        _pricing_date: &OffsetDateTime,
        _forward_curve: Option<Rc<RefCell<ZeroCurve>>>,
        _past_data: Option<Rc<DailyClosePrice>>,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        Err(anyhow!(
            "not supported instrument type on get_floating_cashflows"
        ))
    }

    fn get_fixed_cashflows(
        &self,
        _pricing_date: &OffsetDateTime,
    ) -> Result<HashMap<OffsetDateTime, Real>> {
        Err(anyhow!(
            "not supported instrument type on get_fixed_cashflows"
        ))
    }

    fn get_pricing_date(&self) -> Result<Option<&OffsetDateTime>, anyhow::Error> {
        Err(anyhow!("not supported instrument type on get_pricing_date"))
    }

    fn is_coupon_strip(&self) -> Result<bool> {
        Err(anyhow!("not supported instrument type on is_coupon_strip"))
    }

    fn get_underlying_bonds(&self) -> Result<&Vec<Bond>> {
        Err(anyhow!(
            "not supported instrument type on get_underlying_bonds"
        ))
    }

    fn get_coupon_frequency(&self) -> Result<PaymentFrequency> {
        Err(anyhow!("not supported instrument type on get_frequency"))
    }

    fn get_calendar(&self) -> Result<&JointCalendar> {
        Err(anyhow!("not supported instrument type on get_calendar"))
    }

    fn get_issue_date(&self) -> Result<&OffsetDateTime> {
        Err(anyhow!("not supported instrument type on issue_date"))
    }

    fn get_virtual_bond_npv(&self, _bond_yield: Real) -> Result<Real> {
        Err(anyhow!(
            "not supported instrument type on get_virtual_bond_npv"
        ))
    }

    fn get_schedule(&self) -> Result<&Schedule> {
        Err(anyhow!("not supported instrument type on get_schedule"))
    }

    fn get_fixed_leg_currency(&self) -> Result<&Currency> {
        Err(anyhow!(
            "not supported instrument type on get_fixed_leg_currency"
        ))
    }

    fn get_floating_leg_currency(&self) -> Result<&Currency> {
        Err(anyhow!(
            "not supported instrument type on get_floating_leg_currency"
        ))
    }

    fn get_underlying_currency(&self) -> Result<&Currency> {
        Err(anyhow!(
            "not supported instrument type on get_underlying_currency"
        ))
    }

    fn get_strike(&self) -> Result<Real> {
        Err(anyhow!("not supported instrument type on get_strike"))
    }

    fn get_option_type(&self) -> Result<OptionType> {
        Err(anyhow!("not supported instrument type on get_option_type"))
    }

    fn get_option_daily_settlement_type(&self) -> Result<OptionDailySettlementType> {
        Err(anyhow!(
            "not supported instrument type on get_option_daily_settlement_type"
        ))
    }

    fn get_fxfutres_und_fxcode(&self) -> Result<&FxCode> {
        Err(anyhow!("not supported instrument type on get_fx_code"))
    }

    fn get_floating_to_fixed_fxcode(&self) -> Result<Option<&FxCode>> {
        Err(anyhow!(
            "get_floating_to_fixed_fx allowed only for PlainSwap"
        ))
    }

    fn get_specific_plain_swap_type(&self) -> Result<PlainSwapType> {
        Err(anyhow!(
            "not supported instrument type on get_specific_plain_swap_type"
        ))
    }
}

#[enum_dispatch(InstrumentTrait)]
#[derive(Clone, Debug)]
pub enum Instrument {
    Futures(Futures),
    Bond(Bond),
    BondFutures(BondFutures),
    KTBF(KTBF),
    PlainSwap(PlainSwap),
    FxFutures(FxFutures),
    VanillaOption(VanillaOption),
    Stock(Stock),
    Cash(Cash),
}

/// calculation groups for calculation optimization,
/// On the group, again select calculation sets based on currency and underlying assets (not sub|superset, exact the same assets)
/// currency and underlying_assets categorization
/// GROUP1: Vec<&'static str> = vec!["StockFutures"];
/// GROUP2: Vec<&'static str> = vec!["FixedCouponBond", "BondFutures", "KTBF"];
/// GROUP3: Vec<&'static str> = vec!["StructuredProduct"];
#[derive(Clone, Debug, Default)]
pub struct Instruments {
    instruments: Vec<Rc<Instrument>>,
}

impl Index<usize> for Instruments {
    type Output = Instrument;

    fn index(&self, index: usize) -> &Self::Output {
        &self.instruments[index]
    }
}

impl Instruments {
    pub fn iter(&self) -> std::slice::Iter<'_, Rc<Instrument>> {
        self.instruments.iter()
    }

    pub fn new(instruments: Vec<Rc<Instrument>>) -> Instruments {
        Instruments { instruments }
    }

    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
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
            let names = instrument.get_underlying_codes();
            for name in names.iter() {
                if !underlying_codes.contains(name) {
                    underlying_codes.push(name);
                }
            }
        }
        underlying_codes
    }

    pub fn get_all_fxcodes_for_pricing(&self) -> Vec<FxCode> {
        let mut fxcodes = Vec::<FxCode>::new();
        for instrument in self.instruments.iter() {
            let codes = instrument.get_all_fxcodes_for_pricing();
            for code in codes.iter() {
                if !fxcodes.contains(code) {
                    fxcodes.push(*code);
                }
            }
        }
        fxcodes
    }

    pub fn get_all_quanto_fxcode_und_pairs(&self) -> HashSet<(&String, &FxCode)> {
        let mut fxcodes = HashSet::<(&String, &FxCode)>::new();
        for instrument in self.instruments.iter() {
            let codes = instrument.get_quanto_fxcode_und_pair();
            for code in codes.iter() {
                fxcodes.insert(*code);
            }
        }
        fxcodes
    }

    pub fn get_all_type_names(&self) -> Vec<&'static str> {
        let mut type_names = Vec::<&'static str>::new();
        for instrument in self.instruments.iter() {
            let name = instrument.get_type_name();
            if !type_names.contains(&name) {
                type_names.push(name);
            }
        }
        type_names
    }

    pub fn get_all_currencies(&self) -> Result<Vec<&Currency>> {
        let mut currencies = Vec::<&Currency>::new();
        for instrument in self.instruments.iter() {
            let currency = instrument.get_currency();
            if !currencies.contains(&currency) {
                currencies.push(currency);
            }

            match instrument.get_type_name() {
                "Futures" | "FxFutures" => {
                    let currency = instrument.get_underlying_currency().with_context(|| {
                        anyhow!(
                            "({}:{}) get_underlying_currency failed for {} ({})",
                            file!(),
                            line!(),
                            instrument.get_name(),
                            instrument.get_code(),
                        )
                    })?;
                    if !currencies.contains(&currency) {
                        currencies.push(currency);
                    }
                }
                "PlainSwap" => {
                    let currency = instrument.get_floating_leg_currency().with_context(|| {
                        anyhow!(
                            "({}:{}) get_floating_leg_currency failed for {} ({})",
                            file!(),
                            line!(),
                            instrument.get_name(),
                            instrument.get_code(),
                        )
                    })?;
                    if !currencies.contains(&currency) {
                        currencies.push(currency);
                    }
                }
                _ => {}
            }
        }
        Ok(currencies)
    }

    pub fn instruments_with_underlying(
        &self,
        und_code: &String,
        exclude_type: Option<Vec<&str>>,
    ) -> Vec<Rc<Instrument>> {
        let exclude_type = exclude_type.unwrap_or_default();
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let names = instrument.get_underlying_codes();
            let type_name = instrument.get_type_name();
            if names.contains(&und_code) && !exclude_type.contains(&type_name) {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_currency(&self, currency: &Currency) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            if instrument.get_currency() == currency {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_with_types(&self, type_names: Vec<&str>) -> Vec<Rc<Instrument>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        for instrument in self.instruments.iter() {
            let type_name = instrument.get_type_name();
            if type_names.contains(&type_name) {
                res.push(instrument.clone());
            }
        }
        res
    }

    pub fn instruments_using_curve(
        &self,
        curve_name: &String,
        match_parameter: &MatchParameter,
        exclude_type: Option<Vec<&str>>,
    ) -> Result<Vec<Rc<Instrument>>> {
        let mut res = Vec::<Rc<Instrument>>::new();
        let exclude_type = exclude_type.unwrap_or_default();
        // 1) discount curve
        // 2) collateral curves
        // 3) rate index forward curves
        // borrowing curve can not be hedged, so it skips
        for instrument in self.instruments.iter() {
            if exclude_type.contains(&instrument.get_type_name()) {
                continue;
            }
            // 1)
            if match_parameter.get_discount_curve_name(instrument)? == curve_name {
                res.push(instrument.clone());
            }
            // 2)
            if match_parameter
                .get_collateral_curve_names(instrument)?
                .contains(&curve_name)
            {
                res.push(instrument.clone());
            }
            // 3) forward curve
            if match_parameter.get_rate_index_curve_name(instrument)? == curve_name {
                res.push(instrument.clone());
            }
            // 4) crs curve
            if match_parameter.get_crs_curve_name(instrument)? == curve_name {
                res.push(instrument.clone());
            }
            // 5) floating crs curve
            if match_parameter.get_floating_crs_curve_name(instrument)? == curve_name {
                res.push(instrument.clone());
            }
        }
        Ok(res)
    }

    // all curve names including discount, collateral, and rate index forward curves
    pub fn get_all_curve_names<'a>(
        &'a self,
        match_parameter: &'a MatchParameter,
    ) -> Result<Vec<&String>> {
        let mut res = Vec::<&String>::new();
        let dummy = String::from("Dummy");
        for instrument in self.instruments.iter() {
            let discount_curve_name = match_parameter.get_discount_curve_name(instrument)?;
            if !res.contains(&discount_curve_name) && discount_curve_name != &dummy {
                res.push(discount_curve_name);
            }
            let collateral_curve_names = match_parameter.get_collateral_curve_names(instrument)?;
            for name in collateral_curve_names.iter() {
                if !res.contains(name) && *name != &dummy {
                    res.push(name);
                }
            }
            let rate_index_curve_name = match_parameter.get_rate_index_curve_name(instrument)?;
            if !res.contains(&rate_index_curve_name) && rate_index_curve_name != &dummy {
                res.push(rate_index_curve_name);
            }
            let crs_curve_name = match_parameter.get_crs_curve_name(instrument)?;
            if !res.contains(&crs_curve_name) && crs_curve_name != &dummy {
                res.push(crs_curve_name);
            }
            let floating_crs_curve_name =
                match_parameter.get_floating_crs_curve_name(instrument)?;
            if !res.contains(&floating_crs_curve_name) && floating_crs_curve_name != &dummy {
                res.push(floating_crs_curve_name);
            }
        }
        Ok(res)
    }

    pub fn instruments_with_maturity_upto(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
        maturity: &OffsetDateTime,
        exlucde_type: Option<Vec<&str>>,
    ) -> Vec<Rc<Instrument>> {
        let exlucde_type = exlucde_type.unwrap_or_default();

        match instruments {
            Some(instruments) => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in instruments.iter() {
                    if exlucde_type.contains(&instrument.get_type_name()) {
                        continue;
                    }
                    if let Some(m) = instrument.get_maturity() {
                        if m <= maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
            None => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in self.instruments.iter() {
                    if exlucde_type.contains(&instrument.get_type_name()) {
                        continue;
                    }
                    if let Some(m) = instrument.get_maturity() {
                        if m <= maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
        }
    }

    pub fn instruments_with_maturity_over(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
        maturity: &OffsetDateTime,
        exclude_type: Option<Vec<&str>>,
    ) -> Vec<Rc<Instrument>> {
        let exclude_type = exclude_type.unwrap_or_default();

        match instruments {
            Some(instruments) => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in instruments.iter() {
                    if exclude_type.contains(&instrument.get_type_name()) {
                        continue;
                    }

                    if instrument.get_maturity().is_none() {
                        res.push(instrument.clone());
                    }

                    if let Some(m) = instrument.get_maturity() {
                        if m > maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
            None => {
                let mut res = Vec::<Rc<Instrument>>::new();
                for instrument in self.instruments.iter() {
                    if exclude_type.contains(&instrument.get_type_name()) {
                        continue;
                    }

                    if instrument.get_maturity().is_none() {
                        res.push(instrument.clone());
                    }

                    if let Some(m) = instrument.get_maturity() {
                        if m > maturity {
                            res.push(instrument.clone());
                        }
                    }
                }
                res
            }
        }
    }

    /// This method return the shortest maturity of the given instruments
    /// If instruments is None, it gives the shortest maturity of all instruments
    /// None maturity is taken as an infinite maturity
    /// Therefore, if there is no maturity, it is considered as the longest maturity
    pub fn get_shortest_maturity(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Option<OffsetDateTime> {
        match instruments {
            Some(instruments) => {
                let mut shortest_maturity: Option<OffsetDateTime> = None;
                for instrument in instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if let Some(sm) = shortest_maturity {
                            if *m < sm {
                                shortest_maturity = Some(*m);
                            }
                        } else {
                            shortest_maturity = Some(*m);
                        }
                    }
                }
                shortest_maturity
            }
            None => {
                let mut shortest_maturity: Option<OffsetDateTime> = None;
                for instrument in self.instruments.iter() {
                    if let Some(m) = instrument.get_maturity() {
                        if let Some(sm) = shortest_maturity {
                            if *m < sm {
                                shortest_maturity = Some(*m);
                            }
                        } else {
                            shortest_maturity = Some(*m);
                        }
                    }
                }
                shortest_maturity
            }
        }
    }

    /// This method return the longest maturity of the given instruments
    /// If instruments is None, it gives the longest maturity of all instruments
    /// None maturity is taken as an infinite maturity
    /// Therefore, if there is no maturity, it is considered as the longest maturity
    pub fn get_longest_maturity(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Option<OffsetDateTime> {
        match instruments {
            Some(instruments) => {
                let mut longest_maturity: Option<OffsetDateTime> = None;
                for instrument in instruments.iter() {
                    match instrument.get_maturity() {
                        Some(m) => {
                            if let Some(sm) = longest_maturity {
                                if *m > sm {
                                    longest_maturity = Some(*m);
                                }
                            } else {
                                longest_maturity = Some(*m);
                            }
                        }
                        None => {
                            longest_maturity = None;
                            break;
                        }
                    }
                }
                longest_maturity
            }
            None => {
                let mut longest_maturity: Option<OffsetDateTime> = None;
                for instrument in self.instruments.iter() {
                    match instrument.get_maturity() {
                        Some(m) => {
                            if let Some(sm) = longest_maturity {
                                if *m > sm {
                                    longest_maturity = Some(*m);
                                }
                            } else {
                                longest_maturity = Some(*m);
                            }
                        }
                        None => {
                            longest_maturity = None;
                            break;
                        }
                    }
                }
                longest_maturity
            }
        }
    }

    pub fn get_all_inst_code_clone(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Vec<String> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<String>::new();
                for instrument in instruments.iter() {
                    res.push(instrument.get_code().clone());
                }
                res
            }
            None => {
                let mut res = Vec::<String>::new();
                for instrument in self.instruments.iter() {
                    res.push(instrument.get_code().clone());
                }
                res
            }
        }
    }

    pub fn get_all_unerlying_codes_requiring_volatility(
        &self,
        instruments: Option<&Vec<Rc<Instrument>>>,
    ) -> Vec<String> {
        match instruments {
            Some(instruments) => {
                let mut res = Vec::<String>::new();
                for instrument in instruments.iter() {
                    let names = instrument.get_underlying_codes_requiring_volatility();
                    for name in names.iter() {
                        if !res.contains(name) {
                            res.push(String::clone(name));
                        }
                    }
                }
                res
            }
            None => {
                let mut res = Vec::<String>::new();
                for instrument in self.instruments.iter() {
                    let names = instrument.get_underlying_codes_requiring_volatility();
                    for name in names.iter() {
                        if !res.contains(name) {
                            res.push(String::clone(name));
                        }
                    }
                }
                res
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::currency::{Currency, FxCode};
    use crate::instruments::futures::Futures;
    use crate::instruments::plain_swap::PlainSwap;
    use crate::parameters::rate_index::RateIndex;
    //use crate::enums::RateIndexCode;
    use crate::time::conventions::{BusinessDayConvention, DayCountConvention, PaymentFrequency};
    use crate::time::{
        calendar::Calendar,
        calendars::southkorea::{SouthKorea, SouthKoreaType},
        jointcalendar::JointCalendar,
    };
    use anyhow::Result;
    use time::macros::datetime;

    #[test]
    fn test_instruments() -> Result<()> {
        let fut1 = Futures::new(
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

        let fut2 = Futures::new(
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

        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let _sk = Calendar::SouthKorea(sk);

        let rate_index = RateIndex::new(
            String::from("91D"),
            Currency::KRW,
            String::from("CD 91D"),
            "CD 91D".to_string(),
        )?;

        // make SouthKorea(SouthKorea::Settlement) JointCalendar
        let sk = SouthKorea::new(SouthKoreaType::Settlement);
        let sk = Calendar::SouthKorea(sk);
        let joint_calendar = JointCalendar::new(vec![sk])?;

        let issue_date = datetime!(2021-01-01 09:00:00 UTC);
        let maturity_date = datetime!(2021-12-31 09:00:00 UTC);
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
            Some(rate_index.clone()),
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
            "KRW IRS".to_string(),
            "KRW IRS code".to_string(),
        )?;

        // make Instrument using fut1, fut2, irs
        let instruments = Instruments::new(vec![
            Rc::new(Instrument::Futures(fut1.clone())),
            Rc::new(Instrument::Futures(fut2.clone())),
            Rc::new(Instrument::PlainSwap(irs.clone())),
        ]);

        // make MatchParameter
        let mut collateral_curve_map = HashMap::<String, String>::new();
        let mut rate_index_curve_map = HashMap::<String, String>::new();
        let borrowing_curve_map = HashMap::<String, String>::new();
        let bond_curve_map = HashMap::<(String, IssuerType, CreditRating, Currency), String>::new();

        let mut crs_curve_map = HashMap::<Currency, String>::new();
        // "KOSPI2" -> "KRWGOV"
        // "SPX" -> "USGOV"
        // RateIndexCode::CD -> "KRWIRS"
        collateral_curve_map.insert("KOSPI2".to_string(), "KRWGOV".to_string());
        collateral_curve_map.insert("SPX".to_string(), "USDGOV".to_string());
        rate_index_curve_map.insert("CD 91D".to_string(), "KRWIRS".to_string());
        crs_curve_map.insert(Currency::KRW, "KRWCRS".to_string());
        crs_curve_map.insert(Currency::USD, "USDOIS".to_string());

        let funding_cost_map = HashMap::<Currency, String>::new();
        let crs_curve_map = HashMap::<Currency, String>::new();
        let match_parameter = MatchParameter::new(
            collateral_curve_map,
            borrowing_curve_map,
            bond_curve_map,
            crs_curve_map,
            rate_index_curve_map,
            funding_cost_map,
        );

        // test get_all_underlying_codes
        let underlying_codes = instruments.get_all_underlying_codes();
        assert_eq!(
            underlying_codes,
            vec![&"KOSPI2".to_string(), &"SPX".to_string()]
        );
        // test instruments_with_underlying
        let instruments_with_kospi2 =
            instruments.instruments_with_underlying(&"KOSPI2".to_string(), None);

        assert_eq!(fut1.get_code(), instruments_with_kospi2[0].get_code());
        assert_eq!(fut1.get_name(), instruments_with_kospi2[0].get_name());
        assert_eq!(
            fut1.get_currency(),
            instruments_with_kospi2[0].get_currency()
        );

        // test get_all_curve_names
        let all_curve_names = instruments.get_all_curve_names(&match_parameter)?;
        assert_eq!(all_curve_names, vec![&"KRWGOV", &"USDGOV", &"KRWIRS"]);
        // test instruments_using_curve
        let instruments_using_krw_gov =
            instruments.instruments_using_curve(&"KRWGOV".to_string(), &match_parameter, None)?;

        assert_eq!(fut1.get_code(), instruments_using_krw_gov[0].get_code());

        // test discount curve
        let instruments_using_krw_irs =
            instruments.instruments_using_curve(&"KRWIRS".to_string(), &match_parameter, None)?;

        assert_eq!(irs.get_code(), instruments_using_krw_irs[0].get_code());

        // test instruments_with_currency
        let instruments_with_krw = instruments.instruments_with_currency(&Currency::KRW);
        assert_eq!(fut1.get_code(), instruments_with_krw[0].get_code());
        assert_eq!(irs.get_code(), instruments_with_krw[1].get_code());

        // test instruments_with_type
        let instruments_with_equity_futures = instruments.instruments_with_types(vec!["Futures"]);
        assert_eq!(
            fut1.get_code(),
            instruments_with_equity_futures[0].get_code()
        );
        assert_eq!(
            fut2.get_code(),
            instruments_with_equity_futures[1].get_code()
        );

        let instruments_with_irs = instruments.instruments_with_types(vec!["IRS"]);
        assert_eq!(irs.get_code(), instruments_with_irs[0].get_code());

        // test instruments_with_maturity_upto
        let instruments_with_maturity_upto = instruments.instruments_with_maturity_upto(
            None,
            &datetime!(2022-12-01 09:00:00 UTC),
            None,
        );
        assert_eq!(
            fut1.get_code(),
            instruments_with_maturity_upto[0].get_code()
        );
        assert_eq!(irs.get_code(), instruments_with_maturity_upto[1].get_code());

        Ok(())
    }
}
