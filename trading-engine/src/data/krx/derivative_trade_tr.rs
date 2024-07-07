use crate::data::trade_tick_data::TradeTickData;
use crate::types::{
    venue::Venue,
    base::Slice,
};
use crate::data::krx::converter::{
    KRX_DERIVATIVE_CONVERTER,
    KRX_TIMESTAMP_CONVERTER,
};
use anyhow::Result;
use flexstr::LocalStr;
use std::str::from_utf8_unchecked;

// the elements indicate the location of the beginning and the length of the data
#[derive(Debug, Clone)]
pub struct G703F {
    isin_code_slice: Slice,
    timestamp_slice: Slice,
    //
    trade_price_slice: Slice,
    trade_quantity_slice: Slice,
    //
    near_month_trade_price_slice: Slice,
    far_month_trade_price_slice: Slice,
    //
    trade_type_slice: Slice,
    //
    ask_price_slice_vec: Vec<Slice>,
    bid_price_slice_vec: Vec<Slice>,
    //
    ask_quantity_slice_vec: Vec<Slice>,
    bid_quantity_slice_vec: Vec<Slice>,
    //
    ask_order_count_slice_vec: Vec<Slice>,
    bid_order_count_slice_vec: Vec<Slice>,
    //
}

/* 
TR: G703F 
파생 체결 + 우선호가 (우선호가 5단계)
| 데이터구분값 | 정보구분값  | 정보분배일련번호   | 보드ID | 세션ID | 종목코드 | 정보분배종목인덱스   | 매매처리시각 | 체결가격 | 거래량 | 근월물체결가격 | 원월물체결가격    | 시가 | 고가  | 저가 | 직전가격 | 누적거래량  | 누적거래대금  | 최종매도매수구분코드     | 동적상한가  | 동적하한가  | 매도1단계우선호가가격    | 매수1단계우선호가가격     | 매도1단계우선호가잔량      | 매수1단계우선호가잔량      | 매도1단계우선호가주문건수      |매수1단계우선호가주문건수       |매도2단계우선호가가격     |매수2단계우선호가가격      | 매도2단계우선호가잔량      | 매수2단계우선호가잔량      | 매도2단계우선호가주문건수      | 매수2단계우선호가주문건수      | 매도3단계우선호가가격      | 매수3단계우선호가가격    | 매도3단계우선호가잔량      | 매수3단계우선호가잔량     | 매도3단계우선호가주문건수      | 매수3단계우선호가주문건수       | 매도4단계우선호가가격      | 매수4단계우선호가가격    | 매도4단계우선호가잔량      | 매수4단계우선호가잔량     | 매도4단계우선호가주문건수      | 매수4단계우선호가주문건수      | 매도5단계우선호가가격      | 매수5단계우선호가가격     | 매도5단계우선호가잔량     | 매수5단계우선호가잔량     | 매도5단계우선호가주문건수       | 매수5단계우선호가주문건수       | 매도호가총잔량    | 매수호가총잔량    | 매도호가유효건수    | 매수호가유효건수   |   정보분배메세지종료키워드 |
|-------------|------------|------------------|--------|--------|---------|---------------------|-------------|---------|--------|----------------|----------------|------|------|------|---------|------------|--------------|------------------------|------------|------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|-------------------------|-------------------------|--------------------------|--------------------------|------------------------------|------------------------------|------------------|------------------|-------------------|-------------------|---------------------------|
| String      | String     | Int              | String | String | String  | Int                 | String      | Double  | Int    | Double         | Double         |Double|Double|Double|Double   | Long       | FLOAT128     | String                 | Double     | Double     | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Double                  | Double                  | Int                      | Int                      | Int                          | Int                          | Int              | Int              | Int               | Int               | String                    |
| 2           | 3          | 8                | 2      | 2      | 12      | 6                   | 12          | 9       | 9      | 9              | 9              | 9    | 9    | 9    | 9       | 12         | 22           | 1                      | 9          | 9          | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                       | 9                       | 9                        | 9                        | 5                            | 5                            | 9                | 9                | 5                 | 5                 | 1                         |
| 2           | 5          | 13               | 15     | 17     | 29      | 35                  | 47          | 56      | 65     | 74             | 83             | 92   | 101  | 110  | 119     | 131        | 153          | 154                    | 163        | 172        | 181                     | 190                     | 199                      | 208                      | 213                          | 218                          | 227                     | 236                     | 245                      | 254                      | 259                          | 264                          | 273                     | 282                     | 291                      | 300                      | 305                          | 310                          | 319                     | 328                     | 337                      | 346                      | 351                          | 356                          | 365                     | 374                     | 383                      | 392                      | 397                          | 402                          | 411              | 420              | 425               | 430               | 431                       |

ex) "G703F        G140KR4301V13502001656104939081108000002.12000000005000000.00000000.00000002.83000002.93000002.06000002.11000000021511000000013250790000.0002000006.86000000.01000002.12000002.110000000100000000100000300006000002.13000002.100000000330000000410001100011000002.14000002.090000000290000000430000800010000002.15000002.080000000380000000370000900013000002.16000002.0700000001800000006200007000110000017960000059190049400380" + "255"
*/

impl Default for G703F {
    fn default() -> Self {
        G703F {
            isin_code_slice: Slice { start: 17, end: 29 },
            timestamp_slice: Slice { start: 35, end: 47 },
            //
            trade_price_slice: Slice { start: 47, end: 56 },
            trade_quantity_slice: Slice { start: 56, end: 65 },
            //
            near_month_trade_price_slice: Slice { start: 65, end: 74 },
            far_month_trade_price_slice: Slice { start: 74, end: 83 },
            //
            trade_type_slice: Slice { start: 154, end: 155 },
            //
            ask_price_slice_vec: vec![
                Slice { start: 172, end: 181 },
        }    
}
impl G703F {
    pub fn to_trade_tick_data(data: &[u8]) -> Result<TradeTickData> {
        let data_code = LocalStr::from("G703F");
        let venue = Venue::KRX;
        let isin_code = 

        Ok(
            TradeTickData::default()
        )

    }
}