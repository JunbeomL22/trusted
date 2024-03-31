use std::hash::Hash;
use serde::{Deserialize, Serialize};

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
    ZMW
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