use trading_engine::data::order::{
    LimitOrder,
    MarketOrder,
};
use trading_engine::types::{
    enums::OrderSide,
    isin_code::IsinCode,
    venue::Venue,
};
use trading_engine::orderbook::half_book::HalfBook;
use trading_engine::orderbook::OrderBook;

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
    let ask_levels = vec![102, 101, 100];
    let qtys = vec![1, 2, 3];
    let mut ask_order_vec: Vec<LimitOrder> = Vec::new();
    
    let mut order_id_counter = 0;
    for i in 0..ask_levels.len() {
        for j in qtys.clone().into_iter() {
            ask_order_vec.push(LimitOrder {
                order_id: order_id_counter,
                price: ask_levels[i],
                quantity: j as u64,
                order_side: OrderSide::Ask,
            });
            order_id_counter += 1;
        }
    }

    let bid_levels = vec![99, 98, 97];
    let qtys = vec![1, 2, 3];
    let mut bid_order_vec: Vec<LimitOrder> = Vec::new();
    for i in 0..bid_levels.len() {
        for j in qtys.clone().into_iter() {
            bid_order_vec.push(LimitOrder {
                order_id: order_id_counter,
                price: bid_levels[i],
                quantity: j as u64,
                order_side: OrderSide::Bid,
            });
            order_id_counter += 1;
        }
    }

    let isin_code = IsinCode::new(b"KRXXXXXXXXXX").unwrap();
    let venue = Venue::KRX;
    let mut order_book = OrderBook::initialize_with_isin_venue(isin_code, venue);

    for order in bid_order_vec.clone().into_iter() {
        order_book.add_limit_order(order).unwrap();
    }

    for order in ask_order_vec.clone().into_iter() {
        order_book.add_limit_order(order).unwrap();
    }

    order_book_display(&order_book);

    let market_order = MarketOrder {
        order_id: order_id_counter,
        quantity: 2,
        order_side: OrderSide::Bid,
    };
    order_id_counter += 1;

    println!("Market Order: {:?}", market_order);

    let trades = order_book.process_market_order(&market_order);
    println!("Trades: {:?}", trades);

    order_book_display(&order_book);

    let limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 102,
        quantity: 2,
        order_side: OrderSide::Bid,
    };
    order_id_counter += 1;

    println!("Limit Order: {:?}", limit_order);
    let x = order_book.process_limit_order(limit_order);
    println!("Trades: {:?}", x);

    order_book_display(&order_book);

    let ask_limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 102,
        quantity: 4,
        order_side: OrderSide::Bid,
    };
    order_id_counter += 1;

    println!("Ask Limit Order: {:?}", ask_limit_order);
    let (trades, rem) = order_book.process_limit_order(ask_limit_order).unwrap();
    println!("Trades: {:?}", trades);
    println!("Remaining: {:?}", rem);
    order_book_display(&order_book);

    let bid_limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 101,
        quantity: 6,
        order_side: OrderSide::Bid,
    };
    order_id_counter += 1;

    println!("Bid Limit Order: {:?}", bid_limit_order);
    let (trades, rem) = order_book.process_limit_order(bid_limit_order).unwrap();
    println!("Trades: {:?}", trades);
    println!("Remaining: {:?}", rem);
    order_book_display(&order_book);
    let ask_limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 104,
        quantity: 1,
        order_side: OrderSide::Ask,
    };
    order_id_counter += 1;

    println!("Ask Limit Order: {:?}", ask_limit_order);
    order_book.add_limit_order(ask_limit_order).unwrap();
    order_book_display(&order_book);

    let ask_limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 103,
        quantity: 10,
        order_side: OrderSide::Ask,
    };

    order_id_counter += 1;

    println!("Ask Limit Order: {:?}", ask_limit_order);
    order_book.add_limit_order(ask_limit_order).unwrap();
    order_book_display(&order_book);


    println!("ask summary: ");
    println!("ask best price: {:?}", order_book.asks.best_price);
    println!("ask cache: {:?}", order_book.asks.cache);


    println!("bid summary: ");
    println!("bid best price: {:?}", order_book.bids.best_price);
    println!("bid cache: {:?}", order_book.bids.cache);

    //dbg!(order_book);

}

fn main() {
    half_book_example();
    orderbook_example();
}

fn half_book_example() {
    println!("*********************");
    println!("* HalfBook Example *");
    println!("*********************");
    let price_levels = vec![101, 102, 103];
    let qtys = vec![1, 2, 3];
    let mut order_vec: Vec<LimitOrder> = Vec::new();
    
    let mut order_id_counter = 0;
    for i in 0..price_levels.len() {
        for j in qtys.clone().into_iter() {
            order_vec.push(LimitOrder {
                order_id: order_id_counter,
                price: price_levels[i],
                quantity: j as u64,
                order_side: OrderSide::Bid,
            });
            order_id_counter += 1;
        }
    }

    let mut half_book = HalfBook::initialize(OrderSide::Ask);
    for order in order_vec.clone().into_iter() {
        half_book.add_limit_order(order).unwrap();
    }

    half_book_display(&half_book, None);

    let market_order = MarketOrder {
        order_id: order_id_counter,
        quantity: 2,
        order_side: OrderSide::Bid,
    };
    order_id_counter += 1;

    println!("Market Order: {:?}", market_order);

    let trades = half_book.trade_market_order(&market_order);
    println!("Trades: {:?}", trades);

    half_book_display(&half_book, None);

    let limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 102,
        quantity: 2,
        order_side: OrderSide::Bid,
    };
    order_id_counter += 1;

    println!("Limit Order: {:?}", limit_order);
    let x = half_book.trade_limit_order(&limit_order);
    println!("Trades: {:?}", x);

    half_book_display(&half_book, None);

    let limit_order = LimitOrder {
        order_id: order_id_counter,
        price: 102,
        quantity: 4,
        order_side: OrderSide::Bid,
    };

    let (trades, rem) = half_book.trade_limit_order(&limit_order).unwrap();
    println!("Trades: {:?}", trades);
    println!("Remaining: {:?}", rem);
    half_book_display(&half_book, None);
}