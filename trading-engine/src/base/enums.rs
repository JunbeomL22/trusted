pub trait FromU8 {
    fn from_u8(value: u8) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    NoSide = 0,
    Buy = 1,
    Sell = 2,
}

impl FromU8 for OrderSide {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(OrderSide::NoSide),
            1 => Some(OrderSide::Buy),
            2 => Some(OrderSide::Sell),
            _ => None, 
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookType { 
    L1 = 1,
    L2 = 2,
    L3 = 3,
}

impl FromU8 for BookType {
    fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(BookType::L1),
            2 => Some(BookType::L2),
            3 => Some(BookType::L3),
            _ => None,
        }
    }
}

