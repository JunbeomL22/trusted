use crate::currency::Currency;
use crate::definitions::Real;
use crate::utils::number_format::write_number_with_commas;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use crate::{
    ID,
    InstType,
    AccountingLevel,
};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct InstInfo {
    pub id: ID,
    pub name: String, // "" where not given
    pub inst_type: InstType,
    pub currency: Currency,
    pub unit_notional: Real,
    pub issue_date: Option<OffsetDateTime>,
    pub maturity: Option<OffsetDateTime>,
    pub accounting_level: AccountingLevel,
}

impl Default for InstInfo {
    fn default() -> InstInfo {
        InstInfo {
            id: ID::default(),
            name: "".to_string(),
            inst_type: InstType::default(),
            currency: Currency::default(),
            unit_notional: 1.0,
            issue_date: None,
            maturity: None,
            accounting_level: AccountingLevel::default(),
        }
    }
}

impl std::fmt::Debug for InstInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "    ID: {:?},", self.id)?;
        writeln!(f, "    name: {:?},", self.name)?;
        writeln!(f, "    instrument_type: {:?},", self.inst_type)?;
        writeln!(f, "    currency: {:?},", self.currency)?;
        write!(f, "    unit_notional: ")?;
        write_number_with_commas(f, self.unit_notional)?;
        writeln!(f)?;
        match self.issue_date {
            Some(issue_date) => writeln!(f, "    issue_date: {:?}", issue_date.date()),
            None => writeln!(f, "    issue_date: None"),
        };

        match self.maturity {
            Some(maturity) => writeln!(f, "    maturity: {:?}", maturity.date()),
            None => writeln!(f, "    maturity: None"),
        };

        writeln!(f, "    accounting_level: {:?}", self.accounting_level);

        Ok(())
    }
}

impl InstInfo {
    pub fn new(
        id: ID,
        name: String,
        inst_type: InstType,
        currency: Currency,
        unit_notional: Real,
        issue_date: Option<OffsetDateTime>,
        maturity: Option<OffsetDateTime>,
        accounting_level: AccountingLevel,
    ) -> InstInfo {
        InstInfo {
            id,
            name,
            inst_type,
            currency,
            unit_notional,
            issue_date,
            maturity,
            accounting_level,
        }
    }

    #[inline]
    pub fn type_name(&self) -> &'static str {
        self.inst_type.as_str()
    }

    #[inline]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn symbol_str(&self) -> &str {
        self.id.symbol_str()
    }

    pub fn get_issue_date(&self) -> Option<&OffsetDateTime> {
        self.issue_date.as_ref()
    }

    pub fn get_maturity(&self) -> Option<&OffsetDateTime> {
        self.maturity.as_ref()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        Ticker,
        Venue,
        Symbol,
    };
    use serde_json;

    use crate::currency::Currency;
    use anyhow::Result;

    #[test]
    fn test_instrument_info_serialization() -> Result<()> {
        let ticker = Ticker::new(b"AAPL").unwrap();
        let venue = Venue::KIS;

        let instrument_info = InstInfo {
            id: ID::new(Symbol::Ticker(ticker), venue),
            name: "Apple Inc.".to_string(),
            inst_type: InstType::Stock,
            currency: Currency::USD,
            unit_notional: 1.0,
            issue_date: None,
            maturity: None,
            accounting_level: AccountingLevel::default(),
        };

        let serialized = serde_json::to_string_pretty(&instrument_info).unwrap();

        println!("serialized = {}", serialized);

        let deserialized: InstInfo = serde_json::from_str(&serialized).unwrap();

        println!("deserialized = {:?}", deserialized);

        assert_eq!(instrument_info, deserialized);

        Ok(())
    }
}
