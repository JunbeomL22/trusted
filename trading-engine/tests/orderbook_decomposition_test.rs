#[cfg(test)]
mod tests {
    use trading_engine::orderbook::{
        OrderBook,
        half_book::HalfBook,
    };
    use trading_engine::types::base::{
        VirtualOrderId,
        LevelSnapshot,
    };
    use trading_engine::types::venue::Venue;
    use trading_engine::types::id::isin_code::IsinCode;

    use trading_engine::types::enums::OrderSide;
    use trading_engine::data::order::{
        OrderEnum,
        LimitOrder,
        ModifyOrder,
        CancelOrder,
        NullOrder,
        DecomposedOrder,
    };
    use trading_engine::types::id::{
        ID,
        Symbol,
    };
    use anyhow::Result;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SeqGen {
        seq: u64,
    }

    #[test]
    fn decompose_orderbook() -> Result<()> {
        let mut order_counter = VirtualOrderId { order_id: 0 };
        let ask_levels = vec![102, 101, 100];
        let qtys = vec![1, 2, 3];
        let mut ask_order_vec: Vec<LimitOrder> = Vec::new();
        
        for i in 0..ask_levels.len() {
            for j in qtys.clone().into_iter() {
                ask_order_vec.push(LimitOrder {
                    order_id: order_counter.next_id(),
                    price: ask_levels[i],
                    quantity: j as u64,
                    order_side: OrderSide::Ask,
                });
            }
        }
    
        let bid_levels = vec![99, 98, 97];
        let qtys = vec![1, 2, 3];
        let mut bid_order_vec: Vec<LimitOrder> = Vec::new();
        for i in 0..bid_levels.len() {
            for j in qtys.clone().into_iter() {
                bid_order_vec.push(LimitOrder {
                    order_id: order_counter.next_id(),
                    price: bid_levels[i],
                    quantity: j as u64,
                    order_side: OrderSide::Bid,
                });
            }
        }

        let isin_code = IsinCode::new(b"KRXXXXXXXXXX").unwrap();
        let venue = Venue::KRX;
        let id = ID::new(Symbol::Isin(isin_code.clone()), venue);
        let mut original_orderbook_series = Vec::<OrderBook>::default();
        let mut ask_level_snapshots_series = Vec::<Vec<LevelSnapshot>>::default();
        let mut bid_level_snapshots_series = Vec::<Vec<LevelSnapshot>>::default();
        let mut order_book = OrderBook::initialize_with_id(id);

        original_orderbook_series.push(order_book.clone());
        ask_level_snapshots_series.push(order_book.ask_level_snapshot());
        bid_level_snapshots_series.push(order_book.bid_level_snapshot());


        for order in bid_order_vec.clone().into_iter() {
            order_book.add_limit_order(order).unwrap();
            original_orderbook_series.push(order_book.clone());
            ask_level_snapshots_series.push(order_book.ask_level_snapshot());
            bid_level_snapshots_series.push(order_book.bid_level_snapshot());
        }

        for order in ask_order_vec.clone().into_iter() {
            order_book.add_limit_order(order).unwrap();
            original_orderbook_series.push(order_book.clone());
            ask_level_snapshots_series.push(order_book.ask_level_snapshot());
            bid_level_snapshots_series.push(order_book.bid_level_snapshot());
        }

        for i in 0..ask_order_vec.len() {
            let modify_order = ModifyOrder {
                order_id: ask_order_vec[i].order_id,
                price: ask_order_vec[i].price.saturating_sub(10),
                quantity: ask_order_vec[i].quantity + 1,
            };
            order_book.modify_order(modify_order.clone());
            original_orderbook_series.push(order_book.clone());
            ask_level_snapshots_series.push(order_book.ask_level_snapshot());
            bid_level_snapshots_series.push(order_book.bid_level_snapshot());
        }

        for i in 0..ask_order_vec.len() {
            let cancel_order = CancelOrder {
                order_id: ask_order_vec[i].order_id,
            };
            order_book.cancel_order(cancel_order.clone());
            original_orderbook_series.push(order_book.clone());
            ask_level_snapshots_series.push(order_book.ask_level_snapshot());
            bid_level_snapshots_series.push(order_book.bid_level_snapshot());
        }

        //original_orderbook_series.push(order_book.clone());
        println!("* head original orderbook:\n{}", original_orderbook_series.first().unwrap().to_string());
        println!("* tail original orderbook:\n{}", original_orderbook_series.last().unwrap().to_string());

        let mut book_series = original_orderbook_series.clone();
        let mut order_enum_vec = Vec::<(usize, Vec<DecomposedOrder>)>::default();

        for k in 0..(book_series.len()-1) {
            let order_enum = book_series[k].decomposed_orders_with_update(
                &ask_level_snapshots_series[k+1],
                &bid_level_snapshots_series[k+1],
            )?;
            order_enum_vec.push((k, order_enum));
        }

        
        let mut recovered_orderbook = OrderBook::initialize_with_id(id);
        let mut recovered_orderbook_series = vec![recovered_orderbook.clone()];

        for (_k, decomposed_orders) in order_enum_vec.iter() {
            for decomposed_order in decomposed_orders.iter() {
                match &decomposed_order.order {
                    OrderEnum::LimitOrder(limit_order) => {
                        recovered_orderbook.add_limit_order(limit_order.clone())?;
                    },
                    OrderEnum::RemoveAnyOrder(remove_any_order) => {
                        recovered_orderbook.remove_order(remove_any_order.clone());
                    },
                    _ => {},
                }
            }
            recovered_orderbook_series.push(recovered_orderbook.clone());
        }

        for (k, (original_orderbook, recovered_orderbook)) in original_orderbook_series.iter().zip(recovered_orderbook_series.iter()).enumerate() {
            println!("\n* stage {} *\n", k);
            println!("* original orderbook:\n{}", original_orderbook.to_string());
            println!("* recovered orderbook:\n{}", recovered_orderbook.to_string());
            assert!(
                original_orderbook.eq_level(recovered_orderbook),
                "original orderbook:\n {}\
                recovered orderbook:\n {}",
                original_orderbook.to_string(),
                recovered_orderbook.to_string(),
            );
        }

        Ok(())
    }

    #[test]
    fn decompose_half_book() -> Result<()> {
        println!("We will 1) construct an half book 2) decompose it to orders 3) dockit back");
        let mut id_generator = VirtualOrderId { order_id: 0 };
        //let mut seq_gen = SeqGen::new(0);
        let mut book_series = Vec::<(OrderEnum, HalfBook)>::default();
        let mut half_book = HalfBook::initialize(OrderSide::Ask);

        book_series.push((OrderEnum::NullOrder(NullOrder {}) , half_book.clone()));

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 100,
            quantity: 10,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));        

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 101,
            quantity: 20,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 102,
            quantity: 30,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 99,
            quantity: 40,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 101,
            quantity: 50,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 110,
            quantity: 60,
            order_side: OrderSide::Ask,
        };

        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));

        let limit_order = LimitOrder {
            order_id: id_generator.next_id(),
            price: 100,
            quantity: 200,
            order_side: OrderSide::Ask,
        };
        
        half_book.add_limit_order(limit_order.clone())?;
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::LimitOrder(limit_order.clone()), half_book.clone()));

        let cancel_order = CancelOrder {
            order_id: 1,
        };

        half_book.cancel_order(cancel_order.order_id);
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::CancelOrder(cancel_order.clone()), half_book.clone()));

        let modify_order = ModifyOrder {
            order_id: 2,
            price: 100,
            quantity: 30,
        };

        half_book.change_price(2, 100);
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::ModifyOrder(modify_order.clone()), half_book.clone()));

        let modify_order = ModifyOrder {
            order_id: 3,
            price: 99,
            quantity: 60,
        };

        half_book.change_quantity(3, 60);
        assert!(half_book.check_validity_quantity());
        book_series.push((OrderEnum::ModifyOrder(modify_order.clone()), half_book.clone()));

        let original_book_series = book_series.clone();
        let level_snapshots_series = book_series.iter().map(|(order, book)| {
            let level_snapshots = book.to_level_snapshot();
            level_snapshots
        }).collect::<Vec<Vec<LevelSnapshot>>>();

        let mut order_seq = VirtualOrderId { order_id: 0 };
        let mut order_enum_vec = Vec::<(usize, Vec<OrderEnum>)>::default();
        order_enum_vec.push((0, vec![OrderEnum::NullOrder(NullOrder {})]));
        for k in 0..(book_series.len()-1) {
            let order_enum = book_series[k].1.decomposed_orders_with_update(&level_snapshots_series[k+1], &mut order_seq)?;
            order_enum_vec.push((k, order_enum));
        }

        let mut recovered_half_book = HalfBook::initialize(OrderSide::Ask);
        let mut recovered_half_book_series = Vec::<(OrderEnum, HalfBook)>::default();
        let null_order = OrderEnum::NullOrder(NullOrder {});
        for (i, (k, order_enums)) in order_enum_vec.iter().enumerate() {
            for (j, order_enum) in order_enums.iter().enumerate() {
                match order_enum {
                    OrderEnum::LimitOrder(limit_order) => {
                        recovered_half_book.add_limit_order(limit_order.clone())?;
                    },
                    OrderEnum::RemoveAnyOrder(remove_any_order) => {
                        recovered_half_book.remove_order(remove_any_order.clone());
                    },
                    _ => {},
                }
            }
            recovered_half_book_series.push((null_order.clone(), recovered_half_book.clone()));
        }

        let mut updated_book_series = vec![HalfBook::initialize(OrderSide::Ask)];
        for i in 0..(book_series.len()-1) {
            updated_book_series.push(book_series[i].1.clone());
        }
        for (k, (_order, recovered_book)) in recovered_half_book_series.iter().enumerate() {
            println!("\n* stage {} *\n", k);
            println!("* original book:\n{}", original_book_series[k].1.to_string_upto_depth(None));
            println!("* recovered book:\n {}", recovered_book.to_string_upto_depth(None));
            println!("* updated book:\n {}", updated_book_series[k].to_string_upto_depth(None));
            let mut checker = original_book_series[k].1.eq_level(recovered_book);
            checker = checker && recovered_book.eq_level(&updated_book_series[k]);
            assert!(
                checker,
                "original book:\n {}\
                recovered book:\n {}\
                updated book:\n {}",
                original_book_series[k].1.to_string_upto_depth(None),
                recovered_book.to_string_upto_depth(None),
                updated_book_series[k].to_string_upto_depth(None),
            );
        }

        Ok(())
    }
}