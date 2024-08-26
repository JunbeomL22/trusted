pub mod bond;
pub mod bond_futures;
pub mod cash;
pub mod futures;
pub mod fx_futures;
pub mod inst_info;
pub mod ktbf;
pub mod plain_swap;
pub mod schedule;
pub mod stock;
pub mod vanilla_option;
pub mod id;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum InstType {
    Bond,
    BondFutures,
    Cash,
    Futures,
    FxFutures,
    KTBF,
    PlainSwap,
    Stock,
    VanillaOption,
    ETF,
    CollectiveAsset,
    #[default]
    Undefined,
}

impl InstType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InstType::Bond => "Bond",
            InstType::BondFutures => "BondFutures",
            InstType::Cash => "Cash",
            InstType::Futures => "Futures",
            InstType::FxFutures => "FxFutures",
            InstType::KTBF => "Ktbf",
            InstType::PlainSwap => "PlainSwap",
            InstType::Stock => "Stock",
            InstType::VanillaOption => "VanillaOption",
            InstType::ETF => "ETF",
            InstType::CollectiveAsset => "CollectiveAsset",
            InstType::Undefined => "Undefined",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum AccountingLevel {
    L1 = 1,
    #[default]
    L2 = 2,
    L3 = 3,
}