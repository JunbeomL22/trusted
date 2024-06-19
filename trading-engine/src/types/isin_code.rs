use crate::utils::checkers;
use ustr::Ustr;
use anyhow::{Result, anyhow};
use serde::{
    Serialize, 
    Deserialize,
    de::Deserializer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct IsinCode {
    #[serde(deserialize_with = "from_str")]
    isin: Ustr,
}

impl Serialize for IsinCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
    {
        self.isin.serialize(serializer)
    }
}

fn from_str<'de, D>(deserializer: D) -> Result<Ustr, D::Error>
where D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(Ustr::from(&*s))
}

impl IsinCode {
    pub fn new(isin: &str) -> Result<Self> {
        match checkers::valid_isin_code_length(isin)? &&
                checkers::contains_white_space(isin)? &&
                checkers::is_ascii(isin)? {
            true => Ok(IsinCode {
                isin: Ustr::from(isin),
            }),
            false => {
                let lazy_error = || anyhow!("Invalid ISIN code: {}", isin);
                Err( lazy_error() )
            }
        }
    }

    pub fn as_str(&self) -> &str {
        self.isin.as_str()
    }
}
