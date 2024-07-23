use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogTopic{
    OfiLevelMismatch,
    ZeroQuantity,
}

impl LogTopic {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogTopic::OfiLevelMismatch => "OfiLevelMismatch",
            LogTopic::ZeroQuantity => "ZeroQuantity",
        }
    }
}