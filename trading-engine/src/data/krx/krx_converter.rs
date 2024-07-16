use crate::utils::numeric_converter::{
    IntegerConverter, 
    NumReprCfg, 
    OrderConverter, 
    TimeStampConverter,
    CumQntConverter,
};
use crate::types::isin_code::IsinCode;
use once_cell::sync::Lazy;

impl OrderConverter {
    pub fn krx_risk_free_derivative_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 4,
            decimal_point_length: 4,
            is_signed: true,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let count_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 5,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        let order_count =
            IntegerConverter::new(count_cfg).expect("failed to create order count converter");

        OrderConverter {
            price,
            quantity,
            order_count,
        }
    }
    pub fn krx_derivative_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 3,
            is_signed: true,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let count_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 5,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        let order_count =
            IntegerConverter::new(count_cfg).expect("failed to create order count converter");

        OrderConverter {
            price,
            quantity,
            order_count,
        }
    }

    pub fn krx_stock_converter() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let count_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 5,
            float_normalizer: None,
        };

        let price = IntegerConverter::new(price_cfg).expect("failed to create price converter");

        let quantity =
            IntegerConverter::new(quantity_cfg).expect("failed to create quantity converter");

        let order_count =
            IntegerConverter::new(count_cfg).expect("failed to create order count converter");

        OrderConverter {
            price,
            quantity,
            order_count,
        }
    }
}

impl TimeStampConverter {
    pub fn krx_timestamp_converter() -> TimeStampConverter {
        let time_cfg = NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 12,
            float_normalizer: None,
        };

        let time_converter =
            IntegerConverter::new(time_cfg).expect("failed to create time converter");

        TimeStampConverter {
            converter: time_converter,
        }
    }
}

impl CumQntConverter {
    pub fn krx_cum_qnt_converter() -> CumQntConverter {
        let cfg = NumReprCfg {
            digit_length: 12,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 12,
            float_normalizer: Some(4), // divided by 10,000 while featuring
        };

        let converter = IntegerConverter::new(cfg).expect("failed to create cum qnt converter");

        CumQntConverter { converter }
    }
}

pub static KRX_STOCK_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(OrderConverter::krx_stock_converter);
pub static KRX_RISK_FREE_DERIVATIVE_CONVERTER: Lazy<OrderConverter> = Lazy::new(OrderConverter::krx_risk_free_derivative_converter);
pub static KRX_DERIVATIVE_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(OrderConverter::krx_derivative_converter);
pub static KRX_TIMESTAMP_CONVERTER: Lazy<TimeStampConverter> = Lazy::new(TimeStampConverter::krx_timestamp_converter);
pub static KRX_CUM_QNT_CONVERTER: Lazy<CumQntConverter> = Lazy::new(CumQntConverter::krx_cum_qnt_converter);

#[inline]
#[must_use]
pub fn get_krx_timestamp_converter() -> &'static TimeStampConverter {
    &KRX_TIMESTAMP_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_cum_qnt_converter() -> &'static CumQntConverter {
    &KRX_CUM_QNT_CONVERTER
}

#[inline]
#[must_use]
pub fn get_krx_order_converter(payload: &[u8], isin_code: &IsinCode) -> &'static OrderConverter {
    let und_code = &payload[2..5];
    if [b"04F", b"05F"].contains(und_code) {
        &KRX_STOCK_ORDER_CONVERTER
    } else if isin_code.starts_with(b"KR4169") {
        &KRX_RISK_FREE_DERIVATIVE_CONVERTER
    } else {
        &KRX_DERIVATIVE_ORDER_CONVERTER
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    
    #[test]
    fn test_krx_cum_qnt_converter() -> Result<()> {
        let converter = &KRX_CUM_QNT_CONVERTER;
        let raw = b"123456789012";
        let converted = unsafe { converter.to_cum_qnt_unchecked(raw) };
        assert_eq!(converted, 123456789012);

        let val_f32 = converter.converter.normalized_f32_from_u64(converted);
        assert_eq!((val_f32 - 12345678.9012).abs() < f32::EPSILON, true);

        Ok(())
    }

    #[test]
    fn test_get_order_converter() -> Result<()> {
        let payload = b"XX6FXXXXXXX";
        let isin_code = IsinCode::new(b"KR4169V30013")?;
        let conv = get_krx_order_converter(payload, &isin_code);
        let val = conv.to_book_price(b"-1234.123");

        assert_eq!(val, -1234123);

        Ok(())
    }
}