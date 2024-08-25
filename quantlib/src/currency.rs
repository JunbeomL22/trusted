use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Currency {
    #[default]
    NIL,
    KRW,
    USD,
    EUR,
    JPY,
    CNY,
    CNH,
    GBP,
    AUD,
    CAD,
    CHF,
    NZD,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Implement the formatting logic for Currency here
        write!(f, "{}", self.as_str())
    }
}

//implment from<&str> for Currency
impl From<&str> for Currency {
    fn from(s: &str) -> Self {
        match s {
            "KRW" => Currency::KRW,
            "USD" => Currency::USD,
            "EUR" => Currency::EUR,
            "JPY" => Currency::JPY,
            "CNY" => Currency::CNY,
            "CNH" => Currency::CNH,
            "GBP" => Currency::GBP,
            "AUD" => Currency::AUD,
            "CAD" => Currency::CAD,
            "CHF" => Currency::CHF,
            "NZD" => Currency::NZD,
            _ => Currency::NIL,
        }
    }
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::NIL => "NIL",
            Currency::KRW => "KRW",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::JPY => "JPY",
            Currency::CNY => "CNY",
            Currency::CNH => "CNH",
            Currency::GBP => "GBP",
            Currency::AUD => "AUD",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::NZD => "NZD",
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash, Copy)]
pub struct FxCode {
    currency1: Currency,
    currency2: Currency,
}

impl Serialize for FxCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{:?}_{:?}", self.currency1, self.currency2);
        serializer.serialize_str(&s)
    }
}

impl FxCode {
    pub fn new(currency1: Currency, currency2: Currency) -> FxCode {
        FxCode {
            currency1,
            currency2,
        }
    }

    pub fn get_currency1(&self) -> &Currency {
        &self.currency1
    }

    pub fn get_currency2(&self) -> &Currency {
        &self.currency2
    }

    pub fn reciprocal(self) -> Self {
        FxCode {
            currency1: self.currency2,
            currency2: self.currency1,
        }
    }
}
impl Display for FxCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.currency1.as_str(), self.currency2.as_str())
    }
}

impl Default for FxCode {
    fn default() -> FxCode {
        FxCode {
            currency1: Currency::NIL,
            currency2: Currency::NIL,
        }
    }
}
impl From<&str> for FxCode {
    fn from(code: &str) -> FxCode {
        let currency1 = Currency::from(&code[0..3]);
        let currency2 = Currency::from(&code[3..6]);

        FxCode {
            currency1,
            currency2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{from_str, json, to_string};
    use crate::utils::memory_investigation::print_struct_info;

    #[test]
    fn show_memory_size() {
        print_struct_info(Currency::NIL);
        print_struct_info(&Currency::NIL);
        print_struct_info(FxCode::default());
    }

    #[test]
    fn test_currency_serialization() {
        let currency = Currency::KRW;
        let serialized = to_string(&currency).unwrap();

        assert_eq!(serialized, "\"KRW\"");
        let deserialized: Currency = from_str(&serialized).unwrap();

        assert_eq!(deserialized, currency);
    }

    #[test] // test for make json
    fn test_currency_json() {
        let currency = Currency::KRW;
        let json = json!(currency);

        assert_eq!(json, json!("KRW"));
        let deserialized: Currency = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized, currency);
    }

    #[test] // test for as_str
    fn test_currency_as_str() {
        let currency = Currency::KRW;
        let as_str = currency.as_str();

        assert_eq!(as_str, "KRW");
    }
}
