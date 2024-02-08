pub enum Side {
    NoSide = 0,
    Buy = 1,
    Sell = 2,
}

impl FromU8 for Side {
    fn from_u8(v: u8) -> Result<Side, String> {
        match v {
            0 => Ok(Side::NoSide),
            1 => Ok(Side::Buy),
            2 => Ok(Side::Sell),
            _ => Err(format!("Invalid Side: {}", v)),
        }
    }
}