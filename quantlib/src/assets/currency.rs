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
}