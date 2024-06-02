use crate::currency::Currency;
use time::{Time, UtcOffset};
use crate::instrument::{
    InstrumentTrait,
    Instrument
};

pub struct InstrumentCategory {
    pub type_names: Option<Vec<String>>,
    pub currency: Option<Currency>,
    pub underlying_codes: Option<Vec<String>>,
    pub utc_offset: Option<UtcOffset>,
    pub maturity_close_time: Option<Time>,
}

impl Default for InstrumentCategory {
    fn default() -> InstrumentCategory {
        InstrumentCategory {
            type_names: None,
            currency: None,
            underlying_codes: None,
            utc_offset: None,
            maturity_close_time: None,
        }
    }
}

impl InstrumentCategory {
    pub fn new(
        type_names: Option<Vec<String>>,
        currency: Option<Currency>,
        underlying_codes: Option<Vec<String>>,
        utc_offset: Option<UtcOffset>,
        maturity_close_time: Option<Time>,
    ) -> InstrumentCategory {
        InstrumentCategory {
            type_names,
            currency,
            underlying_codes,
            utc_offset,
            maturity_close_time,
        }
    }

    pub fn contains(instrument: &Instrument) -> bool {
        let instrument_type = instrument.get_type();
        let currency = instrument.get_currency();
        let underlying_codes = instrument.get_underlying_codes();
        let utc_offset = instrument.get_utc_offset();
        let maturity_close_time = instrument.get_maturity_close_time();

        if self.type_names.is_some() {
            if !self.type_names.contains(&instrument_type) {
                return false;
            }
        }

        if self.currency.is_some() {
            if self.currency != currency {
                return false;
            }
        }

        if self.underlying_codes.is_some() {
            if !self.underlying_codes.contains(&underlying_codes) {
                return false;
            }
        }

        if self.utc_offset.is_some() {
            if self.utc_offset != utc_offset {
                return false;
            }
        }

        if self.maturity_close_time.is_some() {
            if self.maturity_close_time != maturity_close_time {
                return false;
            }
        }

        true
    }
}

