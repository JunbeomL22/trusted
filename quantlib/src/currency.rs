use std::hash::Hash;
use std::fmt::Display;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
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
    SEK,
    NOK,
    MXN,
    ZAR,
    BRL,
    INR,
    RUB,
    TRY,
    HKD,
    SGD,
    TWD,
    THB,
    IDR,
    MYR,
    PHP,
    VND,
    PKR,
    LKR,
    BDT,
    NPR,
    KZT,
    UZS,
    KGS,
    TJS,
    MNT,
    AED,
    SAR,
    QAR,
    OMR,
    KWD,
    BHD,
    JOD,
    ILS,
    EGP,
    ZMW,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Implement the formatting logic for Currency here
        write!(f, "{}", self.as_str())
    }
}

impl Default for Currency {
    fn default() -> Currency {
        Currency::NIL
    }
}

//implment from<&str> for Currency
impl From<&str> for Currency {
    fn from(s: &str) -> Self {
        match s {
            "NIL" => Currency::NIL,
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
            "SEK" => Currency::SEK,
            "NOK" => Currency::NOK,
            "MXN" => Currency::MXN,
            "ZAR" => Currency::ZAR,
            "BRL" => Currency::BRL,
            "INR" => Currency::INR,
            "RUB" => Currency::RUB,
            "TRY" => Currency::TRY,
            "HKD" => Currency::HKD,
            "SGD" => Currency::SGD,
            "TWD" => Currency::TWD,
            "THB" => Currency::THB,
            "IDR" => Currency::IDR,
            "MYR" => Currency::MYR,
            "PHP" => Currency::PHP,
            "VND" => Currency::VND,
            "PKR" => Currency::PKR,
            "LKR" => Currency::LKR,
            "BDT" => Currency::BDT,
            "NPR" => Currency::NPR,
            "KZT" => Currency::KZT,
            "UZS" => Currency::UZS,
            "KGS" => Currency::KGS,
            "TJS" => Currency::TJS,
            "MNT" => Currency::MNT,
            "AED" => Currency::AED,
            "SAR" => Currency::SAR,
            "QAR" => Currency::QAR,
            "OMR" => Currency::OMR,
            "KWD" => Currency::KWD,
            "BHD" => Currency::BHD,
            "JOD" => Currency::JOD,
            "ILS" => Currency::ILS,
            "EGP" => Currency::EGP,
            "ZMW" => Currency::ZMW,
            _ => Currency::NIL
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
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::MXN => "MXN",
            Currency::ZAR => "ZAR",
            Currency::BRL => "BRL",
            Currency::INR => "INR",
            Currency::RUB => "RUB",
            Currency::TRY => "TRY",
            Currency::HKD => "HKD",
            Currency::SGD => "SGD",
            Currency::TWD => "TWD",
            Currency::THB => "THB",
            Currency::IDR => "IDR",
            Currency::MYR => "MYR",
            Currency::PHP => "PHP",
            Currency::VND => "VND",
            Currency::PKR => "PKR",
            Currency::LKR => "LKR",
            Currency::BDT => "BDT",
            Currency::NPR => "NPR",
            Currency::KZT => "KZT",
            Currency::UZS => "UZS",
            Currency::KGS => "KGS",
            Currency::TJS => "TJS",
            Currency::MNT => "MNT",
            Currency::AED => "AED",
            Currency::SAR => "SAR",
            Currency::QAR => "QAR",
            Currency::OMR => "OMR",
            Currency::KWD => "KWD",
            Currency::BHD => "BHD",
            Currency::JOD => "JOD",
            Currency::ILS => "ILS",
            Currency::EGP => "EGP",
            Currency::ZMW => "ZMW"
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

    pub fn to_string(&self) -> String {
        format!("{}{}", self.currency1.as_str(), self.currency2.as_str())
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
    use serde_json::{json, to_string, from_str};
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