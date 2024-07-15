use crate::utils::numeric_converter::{
    IntegerConverter, 
    NumReprCfg, 
    OrderConverter, 
    TimeStampConverter,
};
use once_cell::sync::Lazy;

impl OrderConverter {
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

pub static KRX_STOCK_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(OrderConverter::krx_stock_converter);
pub static KRX_DERIVATIVE_ORDER_CONVERTER: Lazy<OrderConverter> = Lazy::new(OrderConverter::krx_derivative_converter);
pub static KRX_TIMESTAMP_CONVERTER: Lazy<TimeStampConverter> = Lazy::new(TimeStampConverter::krx_timestamp_converter);