use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogTopic{
    OfiLevelMismatch,
    ZeroQuantity,
    OrderNotFound,
    UnsupportedDigitSize,
    TooLongDigit,
    UnorderedTimeStamp,
    //
    DateInUtcShift,
}

impl LogTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogTopic::OfiLevelMismatch => "OfiLevelMismatch",
            LogTopic::ZeroQuantity => "ZeroQuantity",
            LogTopic::OrderNotFound => "OrderNotFound",
            LogTopic::UnsupportedDigitSize => "UnsupportedDigitSize",
            LogTopic::TooLongDigit => "TooLongDigit",
            LogTopic::UnorderedTimeStamp => "UnorderedTimeStamp",
            LogTopic::DateInUtcShift => "DateInUtcShift",
        }
    }
}