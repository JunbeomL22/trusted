use crate::instruments::mock_instrument::Mock;
use crate::types::precision::Precision;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub enum Instrument {
    Mock(Mock)
}

impl Instrument {
    pub fn get_price_precision(&self) -> Precision {
        match self {
            Instrument::Mock(mock) => mock.price_precision,
        }
    }

    pub fn get_price_length(&self) -> Precision {
        match self {
            Instrument::Mock(mock) => mock.price_length,
        }
    }

    pub fn get_quantity_precision(&self) -> Precision {
        match self {
            Instrument::Mock(mock) => mock.quantity_precision,
        }
    }

    pub fn get_quantity_length(&self) -> Precision {
        match self {
            Instrument::Mock(mock) => mock.quantity_length,
        }
    }
}
