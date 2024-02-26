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
