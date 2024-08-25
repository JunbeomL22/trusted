use trading_engine::data::order::{
    LimitOrder,
    MarketOrder,
    CancelOrder,
    ModifyOrder,
    RemoveAnyOrder,
};
use trading_engine::types::{
    id::Symbol,
    enums::OrderSide,
    id::isin_code::IsinCode,
    id::InstId,
    venue::Venue,
};
use trading_engine::orderbook::half_book::HalfBook;
use trading_engine::orderbook::OrderBook;
use trading_engine::types::base::VirtualOrderId;

fn half_book_display(half_book: &HalfBook, depth: Option<usize>) {
    let display = half_book.to_string_upto_depth(depth);
    println!("\n{}", display);
}

fn order_book_display(order_book: &OrderBook) {
    let display = order_book.to_string();
    println!("\n{}", display);
}

fn orderbook_example() {
    println!("*********************");
    println!("* OrderBook Example *");
    println!("*********************");

    let mut order_counter = VirtualOrderId::new(0);
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
    let inst_id = InstId::new(Symbol::Isin(isin_code), venue);

    let mut order_book = OrderBook::initialize_with_id(inst_id);

    for order in bid_order_vec.clone().into_iter() {
        order_book.add_limit_order(order).unwrap();
    }

    for order in ask_order_vec.clone().into_iter() {
        order_book.add_limit_order(order).unwrap();
    }

    order_book_display(&order_book);

    let market_order = MarketOrder {
        order_id: order_counter.next_id(),
        quantity: 2,
        order_side: OrderSide::Bid,
    };

    println!("Market Order: {:?}", market_order);

    let trades = order_book.process_market_order(&market_order);
    println!("Trades: {:?}", trades);

    order_book_display(&order_book);

    let limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 102,
        quantity: 2,
        order_side: OrderSide::Bid,
    };
    

    println!("Limit Order: {:?}", limit_order);
    let x = order_book.process_limit_order(limit_order);
    println!("Trades: {:?}", x);

    order_book_display(&order_book);

    let ask_limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 102,
        quantity: 4,
        order_side: OrderSide::Bid,
    };

    println!("Ask Limit Order: {:?}", ask_limit_order);
    let (trades, rem) = order_book.process_limit_order(ask_limit_order).unwrap();
    println!("Trades: {:?}", trades);
    println!("Remaining: {:?}", rem);
    order_book_display(&order_book);

    let bid_limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 101,
        quantity: 6,
        order_side: OrderSide::Bid,
    };

    println!("Bid Limit Order: {:?}", bid_limit_order);
    let (trades, rem) = order_book.process_limit_order(bid_limit_order).unwrap();
    println!("Trades: {:?}", trades);
    println!("Remaining: {:?}", rem);
    order_book_display(&order_book);
    let ask_limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 104,
        quantity: 1,
        order_side: OrderSide::Ask,
    };

    println!("Ask Limit Order: {:?}", ask_limit_order);
    order_book.add_limit_order(ask_limit_order).unwrap();
    order_book_display(&order_book);

    let ask_limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 103,
        quantity: 10,
        order_side: OrderSide::Ask,
    };

    println!("Ask Limit Order: {:?}", ask_limit_order);
    order_book.add_limit_order(ask_limit_order).unwrap();
    order_book_display(&order_book);

    let cancel_order = CancelOrder {
        order_id: 13,
    };

    println!("Cancel Order: {:?}", cancel_order);
    order_book.cancel_order(cancel_order).unwrap();
    order_book_display(&order_book);

    let modify_order = ModifyOrder {
        order_id: 34,
        price: 101,
        quantity: 1,
    };

    println!("Modify Order: {:?}", modify_order);
    let mod_res = order_book.modify_order(modify_order).unwrap();
    println!("Trade history: {:?}", mod_res.0);
    println!("Remaining: {:?}", mod_res.1);

    order_book_display(&order_book);

    let remove_order = RemoveAnyOrder {
        price: 101,
        quantity: 1,
        order_side: OrderSide::Bid,
    };

    println!("Remove Order: {:?}", remove_order);
    let _rem_res = order_book.remove_order(remove_order);
    
    order_book_display(&order_book);

    // dbg!(order_book.clone());
    println!("ask summary: ");
    println!("ask best price: {:?}", order_book.asks.best_price);
    println!("ask cache: {:?}", order_book.asks.cache);
    println!("ask total quantity: {:?}", order_book.asks.get_total_quantity());
    println!("");

    println!("bid summary: ");
    println!("bid best price: {:?}", order_book.bids.best_price);
    println!("bid cache: {:?}", order_book.bids.cache);
    println!("bid total quantity: {:?}", order_book.bids.get_total_quantity());

    //dbg!(order_book);

}

fn main() {
    half_book_example();
    orderbook_example();
}

fn half_book_example() {
    println!("********************");
    println!("* HalfBook Example *");
    println!("********************");
    let price_levels = vec![101, 102, 103];
    let qtys = vec![1, 2, 3];
    let mut order_vec: Vec<LimitOrder> = Vec::new();
    
    let mut order_counter = VirtualOrderId::new(0);
    for i in 0..price_levels.len() {
        for j in qtys.clone().into_iter() {
            order_vec.push(LimitOrder {
                order_id: order_counter.next_id(),
                price: price_levels[i],
                quantity: j as u64,
                order_side: OrderSide::Bid,
            });
        }
    }

    let mut half_book = HalfBook::initialize(OrderSide::Ask);
    for order in order_vec.clone().into_iter() {
        half_book.add_limit_order(order).unwrap();
    }

    half_book_display(&half_book, None);

    let market_order = MarketOrder {
        order_id: order_counter.next_id(),
        quantity: 2,
        order_side: OrderSide::Bid,
    };
    

    println!("Market Order: {:?}", market_order);

    let trades = half_book.trade_market_order(&market_order);
    println!("Trades: {:?}", trades);

    half_book_display(&half_book, None);

    let limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 102,
        quantity: 2,
        order_side: OrderSide::Bid,
    };

    println!("Limit Order: {:?}", limit_order);
    let x = half_book.trade_limit_order(&limit_order);
    println!("Trades: {:?}", x);

    half_book_display(&half_book, None);

    let limit_order = LimitOrder {
        order_id: order_counter.next_id(),
        price: 102,
        quantity: 4,
        order_side: OrderSide::Bid,
    };

    let (trades, rem) = half_book.trade_limit_order(&limit_order).unwrap();
    println!("Trades: {:?}", trades);
    println!("Remaining: {:?}", rem);
    half_book_display(&half_book, None);
}