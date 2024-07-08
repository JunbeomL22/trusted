use crate::utils::numeric_converter::{
    NumReprCfg,
    IntegerConverter,
};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DerivativeConverter {
    price_converter: IntegerConverter,
    quantity_converter: IntegerConverter,
    volume_converter: IntegerConverter,
}

impl Default for DerivativeConverter {
    /* G703F
    | 데이터구분값 | 정보구분값  | 정보분배일련번호   | 보드ID | 세션ID | 종목코드 | 정보분배종목인덱스   | 매매처리시각 | 체결가격 | 거래량 | 근월물체결가격 | 원월물체결가격    | 시가 | 고가  | 저가 | 직전가격 | 누적거래량  | 누적거래대금  | 최종매도매수구분코드     | 동적상한가  | 동적하한가  | 매도1단계우선호가가격    | 매수1단계우선호가가격     | 매도1단계우선호가잔량      | 매수1단계우선호가잔량 | 매도1단계우선호가주문건수 | 매수1단계우선호가주문건수 | 매도2단계우선호가가격 | 매수2단계우선호가가격 | 매도2단계우선호가잔량 | 매수2단계우선호가잔량 | 매도2단계우선호가주문건수 | 매수2단계우선호가주문건수 | 매도3단계우선호가가격 | 매수3단계우선호가가격 | 매도3단계우선호가잔량 | 매수3단계우선호가잔량 | 매도3단계우선호가주문건수 | 매수3단계우선호가주문건수 | 매도4단계우선호가가격 | 매수4단계우선호가가격 | 매도4단계우선호가잔량 | 매수4단계우선호가잔량 | 매도4단계우선호가주문건수 | 매수4단계우선호가주문건수 | 매도5단계우선호가가격 | 매수5단계우선호가가격 | 매도5단계우선호가잔량 | 매수5단계우선호가잔량 | 매도5단계우선호가주문건수 | 매수5단계우선호가주문건수 | 매도호가총잔량 | 매수호가총잔량 | 매도호가유효건수 | 매수호가유효건수 | 정보분배메세지종료키워드 |
    |-------------|------------|------------------|--------|--------|---------|---------------------|-------------|---------|--------|----------------|----------------|------|------|------|---------|------------|--------------|------------------------|------------|------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|------------------|------------------|-------------------|-------------------|---------------------------|
    | String      | String     | Int              | String | String | String  | Int                 | String      | Double  | Int    | Double         | Double         |Double|Double|Double|Double   | Long       | FLOAT128     | String                 | Double     | Double     | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Int              | Int              | Int               | Int               | String                    |
    | 2           | 3          | 8                | 2      | 2      | 12      | 6                   | 12          | 9       | 9      | 9              | 9              | 9    | 9    | 9    | 9       | 12         | 22           | 1                      | 9          | 9          | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                | 9                | 5                 | 5                 | 1                         |
    | 2           | 5          | 13               | 15     | 17     | 29      | 35                  | 47          | 56      | 65     | 74             | 83             | 92   | 101  | 110  | 119     | 131        | 153          | 154                    | 163        | 172        | 181                     | 190                     | 199                      | 208                      | 213                          | 218                          | 227                     | 236                     | 245                      | 254                      | 259                          | 264                          | 273                     | 282                     | 291                      | 300                      | 305                          | 310                          | 319                     | 328                     | 337                      | 346                      | 351                          | 356                          | 365                     | 374                     | 383                      | 392                      | 397                          | 402                          | 411              | 420              | 425               | 430               | 431                       |
     */
    fn default() -> Self {
        let price_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 2,
            is_signed: true,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let volumne_cfg = NumReprCfg {
            digit_length: 9,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 9,
            float_normalizer: None,
        };

        let quantity_cfg = NumReprCfg {
            digit_length: 5,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 5,
            float_normalizer: None,
        };

        let price_converter = IntegerConverter::new(price_cfg)
            .expect("failed to create price converter");

        let quantity_converter = IntegerConverter::new(quantity_cfg)
            .expect("failed to create quantity converter");

        let volume_converter = IntegerConverter::new(volumne_cfg)
            .expect("failed to create order count converter");

        DerivativeConverter {
            price_converter,
            quantity_converter,
            volume_converter,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStampConverter {
    time_converter: IntegerConverter,
}

impl Default for TimeStampConverter {
    fn default() -> Self {
        let time_cfg = NumReprCfg {
            digit_length: 10,
            decimal_point_length: 0,
            is_signed: false,
            drop_decimal_point: false,
            total_length: 10,
            float_normalizer: None,
        };

        let time_converter = IntegerConverter::new(time_cfg)
            .expect("failed to create time converter");

        TimeStampConverter {
            time_converter,
        }
    }
}

unsafe impl Sync for DerivativeConverter {}
unsafe impl Sync for TimeStampConverter {}

pub struct KrxNumericConverter {
    pub derivative_converter: DerivativeConverter,
    pub timestamp_converter: TimeStampConverter,
}
pub static KRX_DERIVATIVE_CONVERTER: Lazy<DerivativeConverter> = Lazy::new(|| DerivativeConverter::default());
pub static KRX_TIMESTAMP_CONVERTER: Lazy<TimeStampConverter> = Lazy::new(|| TimeStampConverter::default());