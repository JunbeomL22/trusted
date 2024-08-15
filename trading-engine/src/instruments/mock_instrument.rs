use crate::instruments::instrument::InstrumentTrait;
use crate::types::{id::isin_code::IsinCode, venue::Venue};
use crate::utils::numeric_converter::NumReprCfg;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct MockInstrument {
    ising_code: IsinCode,
    venue: Venue,
    //
    unit_amount: u32,
    //
    last_trade_date: OffsetDateTime,
    last_trade_date_unix_nano: u64,
    //
    price_repr_cfg: NumReprCfg,
    //
    quantity_repr_cfg: NumReprCfg,
}

impl Default for MockInstrument {
    fn default() -> MockInstrument {
        MockInstrument {
            ising_code: IsinCode::default(),
            venue: Venue::default(),
            unit_amount: 0,
            last_trade_date: OffsetDateTime::now_utc(),
            last_trade_date_unix_nano: 0,
            price_repr_cfg: NumReprCfg::default(),
            quantity_repr_cfg: NumReprCfg::default(),
        }
    }
}

impl MockInstrument {
    pub fn new(
        ising_code: IsinCode,
        venue: Venue,
        unit_amount: u32,
        last_trade_date: OffsetDateTime,
        last_trade_date_unix_nano: u64,
        price_repr_cfg: NumReprCfg,
        quantity_repr_cfg: NumReprCfg,
    ) -> MockInstrument {
        MockInstrument {
            ising_code,
            venue,
            unit_amount,
            last_trade_date,
            last_trade_date_unix_nano,
            price_repr_cfg,
            quantity_repr_cfg,
        }
    }
}

impl InstrumentTrait for MockInstrument {
    #[inline]
    #[must_use]
    fn get_venue(&self) -> Venue {
        self.venue
    }
}
