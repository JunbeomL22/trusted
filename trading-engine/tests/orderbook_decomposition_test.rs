#[cfg(test)]
mod tests {
    use trading_engine::orderbook::{
        OrderBook,
        half_book::HalfBook,
    };
    use trading_engine::types::base::{
        BookPrice,
        BookQuantity,
        OrderId,
        VirtualOrderId,
    };

    use trading_engine::types::enums::OrderSide;
    use trading_engine::data::order::{
        OrderEnum,
        LimitOrder,
        MarketOrder,
        RemoveAnyOrder,
        DecomposedOrder,
    };
    use anyhow::Result;
    use rustc_hash::FxHashMap;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SeqGen {
        seq: u64,
    }

    impl SeqGen {
        pub fn new() -> Self {
            SeqGen { seq: 0 }
        }
        pub fn next(&mut self) -> u64 {
            let res = self.seq;
            self.seq += 1;
            res
        }
    }

    #[test]
    fn decompose_half_book() -> Result<()> {
        println!("We will 1) construct an half book 2) decompose it to orders 3) dockit back");
        let mut id_generator = VirtualOrderId { order_id: 0 };
        let mut seq_gen = SeqGen::new();
        let mut book_series = FxHashMap::<u64, HalfBook>::default();
        let mut half_book = HalfBook::initialize(OrderSide::Ask);
        book_series.insert(0, half_book.clone());
        let limit_order = LimitOrder {
            order_id: id_generator.next(),
            price: 100,
            quantity: 10,
            order_side: OrderSide::Ask,
        };
        println!("order1 {:?}", limit_order);
        half_book.add_limit_order(limit_order)?;
        book_series.push(half_book.clone());
        println!("half_book {}", half_book.to_string_upto_depth(None));
        dbg!(book_series.clone());
        Ok(())
    }
}