use crate::utils::numeric_converter::{
    NumReprCfg,
    IntegerConverter,
    OrderConverter,
    TimeStampConverter,
};
use crate::types::base::{
    BookPrice,
    BookQuantity,
    OrderCount,
};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

pub static KRX_DERIVATIVE_CONVERTER: Lazy<OrderConverter> = Lazy::new(|| {
    let price_cfg = NumReprCfg {
        digit_length: 5,
        decimal_point_length: 2,
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

    let price = IntegerConverter::new(price_cfg)
        .expect("failed to create price converter");

    let quantity = IntegerConverter::new(quantity_cfg)
        .expect("failed to create quantity converter");

    let order_count = IntegerConverter::new(count_cfg)
        .expect("failed to create order count converter");

    OrderConverter {
        price,
        quantity,
        order_count,
    }
});


pub static KRX_TIMESTAMP_CONVERTER: Lazy<TimeStampConverter> = Lazy::new(|| {
    let time_cfg = NumReprCfg {
        digit_length: 12,
        decimal_point_length: 0,
        is_signed: false,
        drop_decimal_point: false,
        total_length: 12,
        float_normalizer: None,
    };

    let time_converter = IntegerConverter::new(time_cfg)
        .expect("failed to create time converter");

    TimeStampConverter { converter: time_converter }
});
