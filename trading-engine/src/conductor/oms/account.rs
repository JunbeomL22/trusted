use crate::InstId;
use crate::Real;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use crate::types::inst_info::BaseInstMap;
use crate::BookQuantity;
use crate::types::enums::OrderSide;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Side {
    #[default]
    Buy,
    Sell,
}

impl Side {
    #[inline]
    pub fn opposite(&self) -> Self {
        match self {
            Side::Buy => Side::Sell,
            Side::Sell => Side::Buy,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Balance {
    pub side: Side,
    pub outstanding: BookQuantity,
    pub pending_order: BookQuantity,
    pub liquidatable: BookQuantity,
    pub outstanding_upper_bound: Option<BookQuantity>,
    pub outstanding_lower_bound: Option<BookQuantity>,
}

impl Balance {
    pub fn initialize(side: Side, outstanding: BookQuantity) -> Self {
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

    pub fn order_available(&self, side: OrderSide, inst_id: InstId, base_inst_map: &BaseInstMap) -> BookQuantity {
        let base_inst = base_inst_map.get(inst_id).unwrap();
        let mut available = self.liquidatable;
        if let Some(upper_bound) = self.outstanding_upper_bound {
            available = available.min(upper_bound - self.outstanding);
        }
        if let Some(lower_bound) = self.outstanding_lower_bound {
            available = available.max(lower_bound - self.outstanding);
        }
        match side {
            OrderSide::Buy => available.min(base_inst.max_buy),
            OrderSide::Sell => available.min(base_inst.max_sell),
        }
    }


}