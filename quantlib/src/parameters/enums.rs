use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum ZeroCurveCode {
    Undefined = 0,
    KRWGOV = 1,
    KRWIRS = 2,
    KRWOIS = 3,
    KRWCRS = 4,
    USDGOV = 5,
    USDIRS = 6,
    USDOIS = 7,
    KSD = 8, // KOFR -5bp
}

impl fmt::Display for ZeroCurveCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ZeroCurveCode::Undefined => write!(f, "Undefined"),
            ZeroCurveCode::KRWGOV => write!(f, "KRWGOV"),
            ZeroCurveCode::KRWIRS => write!(f, "KRWIRS"),
            ZeroCurveCode::KRWOIS => write!(f, "KRWOIS"),
            ZeroCurveCode::KRWCRS => write!(f, "KRWCRS"),
            ZeroCurveCode::USDGOV => write!(f, "USDGOV"),
            ZeroCurveCode::USDIRS => write!(f, "USDIRS"),
            ZeroCurveCode::USDOIS => write!(f, "USDOIS"),
            ZeroCurveCode::KSD => write!(f, "KSD"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Compounding {
    Simple = 0,
    Continuous = 1,
}


pub enum CrateRating {
    Undefined = 0,
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
}

pub enum IssuerType {
    Undefined = 0,
    Government = 1,
    Public = 2,
    Corporate = 3,
    Financial = 4,
    Sovereign = 5,
}