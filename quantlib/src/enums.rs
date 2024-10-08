use serde::{Deserialize, Serialize};
//use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum Compounding {
    Simple = 0,
    Continuous = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum CreditRating {
    None = 0,
    AAA = 1,
    AAp = 2,
    AA = 3,
    AAm = 4,
    Ap = 5,
    A = 6,
    Am = 7,
    BBBp = 8,
    BBB = 9,
    BBBm = 10,
    BBp = 11,
    BB = 12,
    BBm = 13,
    Bp = 14,
    B = 15,
    C = 16,
    D = 17,
    Undefined = 18,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum IssuerType {
    None = 0,
    Government = 1,
    Public = 2,
    CorporateGuaranteed = 3,
    CorporateUnguaranteed = 4,
    Financial = 5,
    Undefined = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum RankType {
    None = 0,
    Senior = 1,
    Subordinated = 2,
    Junior = 3,
    Mezzanine = 4,
    Equity = 5,
    Undefined = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum AccountingLevel {
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub enum OptionExerciseType {
    European,
    American,
    Bermudan,
}

/// option daily settlement type.
/// HKEX settles the amount of option MtM on a daily basis, as in Futures.
/// KRX, Eurex, CME, and OKX does not settle the amount of option MtM on a daily basis.
/// If it is settled, the option value does not need to be discounted, again as in Futures.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub enum OptionDailySettlementType {
    Settled,
    NotSettled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy, Default)]
pub enum StickynessType {
    #[default]
    StickyToMoneyness,
    StickyToStrike,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum VanillaOptionCalculationMethod {
    MonteCarlo = 0,
    FiniteDifference = 1,
    Analytic = 2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum StockRankType {
    Common = 0,
    Preferred = 1,
    Warrant = 2,
    Convertible = 3,
    Undefined = 4,
}
