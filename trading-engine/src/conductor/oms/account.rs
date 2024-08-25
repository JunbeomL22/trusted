use crate::InstId;
use crate::Real;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use crate::types::inst_info::BaseInstMap;
use crate::BookQuantity;
use crate::types::enums::OrderSide;


impl OrderSide {
    #[inline]
    pub fn opposite(&self) -> Self {
        match self {
            OrderSide::Bid => OrderSide::Ask,
            OrderSide::Ask => OrderSide::Bid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Balance {
    pub side: OrderSide,
    pub outstanding: BookQuantity,
    pub pending_order: BookQuantity,
    pub liquidatable: BookQuantity,
    pub outstanding_upper_bound: Option<BookQuantity>,
    pub outstanding_lower_bound: Option<BookQuantity>,
}

impl Balance {
    pub fn initialize(side: OrderSide, outstanding: BookQuantity) -> Self {
        Balance {
            side,
            outstanding,
            pending_order: 0,
            liquidatable: outstanding,
            outstanding_upper_bound: None,
            outstanding_lower_bound: None,
        }
    }

    pub fn without_short_sell(&mut self) -> &mut Self {
        self.outstanding_lower_bound = Some(0);
        self
    }

    pub fn with_upper_bound(&mut self, upper_bound: BookQuantity) -> &mut Self {
        self.outstanding_upper_bound = Some(upper_bound);
        self
    }

    pub fn with_lower_bound(&mut self, lower_bound: BookQuantity) -> &mut Self {
        self.outstanding_lower_bound = Some(lower_bound);
        self

    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.outstanding == 0
    }
}