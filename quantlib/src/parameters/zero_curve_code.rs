use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub enum ZeroCurveCode {
    Credit = 0,
    KRWGOV = 1, 
    KRWIRS = 2, 
    KRWOIS = 3,
    KRWCRS = 4,
    USDGOV = 5,
    USDIRS = 6,
    USDOIS = 7,
    KSD = 8, // KOFR -5bp
    EURIRS = 9,
    EUROIS = 10,
    EURGOV = 11,
    EURCRS = 12,
    CNYIRS = 13,
    CNYOIS = 14,
    CNYGOV = 15,
    CNYCRS = 16,
    JPYIRS = 17,
    JPYOIS = 18,
    JPYGOV = 19,
    JPYCRS = 20,
    HKDIRS = 21,
    HKDOIS = 22,
    HKDGOV = 23,
    HKDCRS = 24, 
    KDB = 25,
    MSB = 26,
    Undefined = 27,
}

macro_rules! zero_curve_code_from_str {
    ($($variant:ident),* $(,)?) => {
        impl ZeroCurveCode {
            pub fn from_str(code: &str) -> Option<Self> {
                match code {
                    $(
                        stringify!($variant) => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }
        }
    };
}

macro_rules! zero_curve_code_to_str {
    ($($variant:ident),* $(,)?) => {
        impl ZeroCurveCode {
            pub fn to_str(&self) -> &'static str {
                match self {
                    $(
                        Self::$variant => stringify!($variant),
                    )*
                }
            }
        }
    };
}

zero_curve_code_from_str! {
    Credit,
    KRWGOV,
    KRWIRS,
    KRWOIS,
    KRWCRS,
    USDGOV,
    USDIRS,
    USDOIS,
    KSD,
    EURIRS,
    EUROIS,
    EURGOV,
    EURCRS,
    CNYIRS,
    CNYOIS,
    CNYGOV,
    CNYCRS,
    JPYIRS,
    JPYOIS,
    JPYGOV,
    JPYCRS,
    HKDIRS,
    HKDOIS,
    HKDGOV,
    HKDCRS,
    KDB,
    MSB,
    Undefined,
}

zero_curve_code_to_str! {
    Credit,
    KRWGOV,
    KRWIRS,
    KRWOIS,
    KRWCRS,
    USDGOV,
    USDIRS,
    USDOIS,
    KSD,
    EURIRS,
    EUROIS,
    EURGOV,
    EURCRS,
    CNYIRS,
    CNYOIS,
    CNYGOV,
    CNYCRS,
    JPYIRS,
    JPYOIS,
    JPYGOV,
    JPYCRS,
    HKDIRS,
    HKDOIS,
    HKDGOV,
    HKDCRS,
    KDB,
    MSB,
    Undefined,
}
