use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Currency {
    NIL,
    KRW,
    USD,
    EUR,
    JPY,
    CNY,
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

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Currency::NIL => "NIL",
            Currency::KRW => "KRW",
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::JPY => "JPY",
            Currency::CNY => "CNY",
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