//
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum RateIndexCode {
    Dummy= 0,
    HIBOR = 1,
    SOFR = 2,
    CD = 3,
    KOFR = 4,
    ESTR = 5,
    TONAR = 6,
    HONIA = 7,
    LIBOR = 8,
    Undefined = 9,
}

impl fmt::Display for RateIndexCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RateIndexCode::Dummy => write!(f, "Dummy"),
            RateIndexCode::HIBOR => write!(f, "HIBOR"),
            RateIndexCode::SOFR => write!(f, "SOFR"),
            RateIndexCode::CD => write!(f, "CD"),
            RateIndexCode::KOFR => write!(f, "KOFR"),
            RateIndexCode::ESTR => write!(f, "ESTR"),
            RateIndexCode::TONAR => write!(f, "TONAR"),
            RateIndexCode::HONIA => write!(f, "HONIA"),
            RateIndexCode::LIBOR => write!(f, "LIBOR"),
            RateIndexCode::Undefined => write!(f, "Undefined"),
        }
    }
}

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


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum OptionType {
    Call,
    Put,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum OptionExerciseType {
    European,
    American,
    Bermudan,
}