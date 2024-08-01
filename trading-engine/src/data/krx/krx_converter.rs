use crate::utils::numeric_converter::{
    IntegerConverter, 
    NumReprCfg, 
    OrderConverter, 
    TimeStampConverter,
    CumQntConverter,
    YieldConverter,
    OrderCounter,
};
use crate::types::isin_code::IsinCode;
use crate::types::timestamp::{
    DateUnixNanoGenerator,
    TimeStamp,
    UnixNano,
    HOUR_NANOSCALE,
};
//
use once_cell::sync::Lazy;
impl OrderCounter {
    pub fn krx_base_order_counter() -> Self {
        let cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 5,
            float_normalizer: None,
        };

        let order_count = IntegerConverter::new(cfg).expect("failed to create order counter converter");

        OrderCounter { order_count }
    }
}

impl OrderConverter {
    pub fn krx_risk_free_derivative_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 4,
            decimal_point_length: 4,
            is_signed: true,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }
    pub fn krx_base_derivative_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 3,
            is_signed: true,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }

    pub fn krx_stock_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 2,
            drop_decimal_point: false,
            total_length: 11,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 12,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }

    pub fn krx_stock_derivative_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 8,
            decimal_point_length: 0,
            is_signed: true,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: Some(1),
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }

    /// Pork or gold derivatives excluding fx derivatives
    pub fn krx_general_commodity_derivative_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 8,
            decimal_point_length: 0,
            is_signed: true,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: Some(1),
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }

    pub fn krx_repo_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 6,
            decimal_point_length: 4,
            is_signed: true,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 11,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 15,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 15,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }
    pub fn krx_base_bond_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 7,
            decimal_point_length: 3,
            is_signed: false,
            unused_length: 1,
            drop_decimal_point: false,
            total_length: 11,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 12,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        OrderConverter {
            price,
            quantity,
        }
    }
}

impl TimeStampConverter {
    pub fn krx_timestamp_converter() -> TimeStampConverter {
        let time_cfg = NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 12,
            float_normalizer: None,
        };

        let converter =
            IntegerConverter::new(time_cfg).expect("failed to create time converter");

        let offset_nano = 9 * HOUR_NANOSCALE;
        TimeStampConverter::new(converter, offset_nano)
    }
}

impl CumQntConverter {
    pub fn krx_base_cum_qnt_converter() -> CumQntConverter {
        let cfg = NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 12,
            float_normalizer: Some(4), // divided by 10,000 while featuring
        };

        let converter = IntegerConverter::new(cfg).expect("failed to create cum qnt converter");

        CumQntConverter { converter }
    }

    pub fn krx_bond_cum_qnt_converter() -> CumQntConverter {
        let cfg = NumReprCfg {
            digit_length: 15,
            decimal_point_length: 0,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 15,
            float_normalizer: Some(4), // divided by 10,000 while featuring
        };

        let converter = IntegerConverter::new(cfg).expect("failed to create cum qnt converter");

        CumQntConverter { converter }
    }

    pub fn krx_cum_trd_value_converter() -> CumQntConverter {
        let cfg = NumReprCfg {
            digit_length: 18,
            decimal_point_length: 4,
            is_signed: false,
            unused_length: 0,
            drop_decimal_point: true,
            total_length: 22,
            float_normalizer: Some(8), // divided by 10^8 while featuring
        };

        let converter = IntegerConverter::new(cfg).expect("failed to create cum qnt converter");

        CumQntConverter { converter }
    }
}

impl YieldConverter {
    pub fn krx_yield_converter() -> YieldConverter {
        let cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 7,
            is_signed: true,
            unused_length: 0,
            drop_decimal_point: false,
            total_length: 13,
            float_normalizer: None,
        };

        let converter = IntegerConverter::new(cfg).expect("failed to create yield converter");

        YieldConverter { converter }
    }
}

#[inline]
#[must_use]
pub fn get_krx_timestamp_converter() -> &'static TimeStampConverter {
    static KRX_TIMESTAMP_CONVERTER: Lazy<TimeStampConverter> = Lazy::new(
        TimeStampConverter::krx_timestamp_converter);
    &KRX_TIMESTAMP_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_base_cum_qnt_converter() -> &'static CumQntConverter {
    static KRX_BASE_CUM_QNT_CONVERTER: Lazy<CumQntConverter> = Lazy::new(
        CumQntConverter::krx_base_cum_qnt_converter);
    &KRX_BASE_CUM_QNT_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_bond_cum_qnt_converter() -> &'static CumQntConverter {
    static KRX_BOND_CUM_QNT_CONVERTER: Lazy<CumQntConverter> = Lazy::new(
        CumQntConverter::krx_bond_cum_qnt_converter);
    &KRX_BOND_CUM_QNT_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_stock_order_converter() -> &'static OrderConverter {
    static KRX_STOCK_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_stock_converter);
    &KRX_STOCK_ORDER_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_base_bond_order_converter() -> &'static OrderConverter {
    static KRX_BASE_BOND_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_base_bond_converter);
    &KRX_BASE_BOND_ORDER_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_repo_order_converter() -> &'static OrderConverter {
    static KRX_REPO_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_repo_converter);
    &KRX_REPO_ORDER_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_yield_converter() -> &'static YieldConverter {
    static KRX_YIELD_CONVERTER: Lazy<YieldConverter> = Lazy::new(
        YieldConverter::krx_yield_converter);
    &KRX_YIELD_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_base_order_counter() -> &'static OrderCounter {
    static KRX_BASE_ORDER_COUNTER: Lazy<OrderCounter> = Lazy::new(
        OrderCounter::krx_base_order_counter);
    &KRX_BASE_ORDER_COUNTER
}

#[inline]
#[must_use]
pub fn get_krx_derivative_converter(payload: &[u8], isin_code: &IsinCode) -> &'static OrderConverter {
    static KRX_STOCK_DERIVATIVE_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_stock_derivative_converter);

    static KRX_GENERAL_COMM_DERIVATIVE_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_general_commodity_derivative_converter);

    static KRX_RISK_FREE_DERIVATIVE_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_risk_free_derivative_converter);
        
    static KRX_BASE_DERIVATIVE_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(
        OrderConverter::krx_base_derivative_converter);

    static TR_STOCK: [&'static [u8]; 2] = [b"04F", b"05F"];
    static TR_COMMODITY: &'static [u8] = b"10F";
    static RISK_FREE_ISIN_HEADER: &'static [u8] = b"KR4169";

    let tr_und_code = &payload[2..5];
    if TR_STOCK.contains(&tr_und_code) {
        return &KRX_STOCK_DERIVATIVE_ORDER_CONVERTER;
    } else if tr_und_code == TR_COMMODITY {
        return &KRX_GENERAL_COMM_DERIVATIVE_ORDER_CONVERTER;
    } else if isin_code.starts_with(RISK_FREE_ISIN_HEADER) {
        return &KRX_RISK_FREE_DERIVATIVE_CONVERTER;
    } else {
        return &KRX_BASE_DERIVATIVE_ORDER_CONVERTER;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    
    #[test]
    fn test_krx_cum_qnt_converter() -> Result<()> {
        let converter = get_krx_base_cum_qnt_converter();
        let raw = b"123456789012";
        let converted = unsafe { converter.to_cum_qnt_unchecked(raw) };
        assert_eq!(converted, 123456789012);

        let val_f32 = converter.converter.normalized_real_from_u64(converted);
        assert_eq!((val_f32 - 12345678.9012).abs() < 5e-7, true);

        Ok(())
    }

    #[test]
    fn test_get_order_converter() -> Result<()> {
        let payload = b"XX6FXXXXXXX";
        let isin_code = IsinCode::new(b"KR4169V30013")?;
        let conv = get_krx_derivative_converter(&payload[..5], &isin_code);
        let val = conv.to_book_price(b"-1234.123");

        assert_eq!(val, -1234123);

        Ok(())
    }
}