use trading_engine::orderbook::half_book::HalfBook;
use trading_engine::types::enums::OrderSide;
use trading_engine::data::order::{
    LimitOrder,
    MarketOrder,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_orderbook(c: &mut Criterion) {
    let n = 10_000;
    let price_levels = 50;
    let mut order_vec: Vec<LimitOrder> = Vec::with_capacity(n+1);
    for i in 0..(n / price_levels) {
        for j in 0..price_levels {
            order_vec.push(LimitOrder {
                order_id: i as u64,
                price: (j+1) as i64,
                quantity: (i+1) as u64,
                order_side: OrderSide::Bid,
            });
        }
    }

    let mut half_book = HalfBook::initialize(OrderSide::Ask);
    let mut group = c.benchmark_group("orderbook");

    group.sample_size(20);

    group.bench_function(format!("clone {n} elements order_vec").as_str(), |b| {
        b.iter(|| {
            black_box(order_vec.clone());
        })
    });

    group.bench_function(format!("add {n} orders with {price_levels} price levels").as_str(), |b| {
        b.iter(|| {
            for order in order_vec.clone().into_iter() {
                half_book.add_limit_order(black_box(order)).unwrap();
            }
        })
    });

    group.bench_function(format!("change_price {n} orders").as_str(), |b| {
        b.iter(|| {
            for order in order_vec.iter() {
                half_book.change_price(black_box(order.order_id), black_box(order.price+1));
            }
        })
    });

    group.bench_function(format!("change_quantity {n} orders").as_str(), |b| {
        b.iter(|| {
            for order in order_vec.iter() {
                half_book.change_quantity(black_box(order.order_id), black_box(order.quantity+1));
            }
        })
    });

    group.bench_function(format!("now cancel all {n} orders").as_str(), |b| {
        b.iter(|| {
            for order in order_vec.iter() {
                half_book.cancel_order(black_box(order.order_id));
            }
        })
    });

    group.finish();
}

// bench cancel, change price and quantity in the middle of the book
// where the orderbook is very big 50 prices levels and each 1,000 orders
fn bench_stress_test(c: &mut Criterion) {
    let n = 9_998;
    let price_levels = 50;
    let mut order_vec: Vec<LimitOrder> = Vec::with_capacity(n+1);
    for i in 0..(n / price_levels) {
        for j in 0..price_levels {
            order_vec.push(LimitOrder {
                order_id: i as u64 *price_levels as u64+ j as u64,
                price: (j+1) as i64,
                quantity: (i+1) as u64,
                order_side: OrderSide::Bid,
            });
        }
    }

    let test_id1 = 5_500;
    let test_id2 = 5_501;
    //
    let mut half_book = HalfBook::initialize(OrderSide::Bid);
    for order in order_vec.clone().into_iter() {
        half_book.add_limit_order(order).unwrap();
    }

    let mut group = c.benchmark_group("stress_test");

    group.sample_size(30);

    group.bench_function(format!("change_quantity an order in the middle of the book").as_str(), |b| {
        b.iter(|| {
            half_book.change_quantity(test_id1, black_box(1000));
            //half_book.change_quantity(test_id2, black_box(10000));
        })
    });

    group.bench_function(format!("change_price an order in the middle of the book").as_str(), |b| {
        b.iter(|| {
            half_book.change_price(test_id1, black_box(1));
            //half_book.change_price(test_id2, black_box(151));
            
            
        })
    });


    let n = 2;
    group.bench_function(format!("add new {n} orders with fresh price").as_str(), |b| {
        b.iter(|| {
            for i in 0..n {
                half_book.add_limit_order(LimitOrder {
                    order_id: i + 100_000 as u64,
                    price: 150,
                    quantity: 1000,
                    order_side: OrderSide::Bid,
                }).unwrap();
            }
        })
    });

    group.bench_function(format!("add new {n} orders in existing price").as_str(), |b| {
        b.iter(|| {
            for i in 0..n {
                half_book.add_limit_order(LimitOrder {
                    order_id: i + 100_000 as u64,
                    price: 25,
                    quantity: 1000,
                    order_side: OrderSide::Bid,
                }).unwrap();
            }
        })
    });
    group.bench_function(format!("cancel {n} orders in the middle of the book").as_str(), |b| {
        b.iter(|| {
            half_book.cancel_order(black_box(test_id1));
            half_book.cancel_order(black_box(test_id2));
        })
    });
    
    let market_order = MarketOrder {
        order_id: 1,
        quantity: 100,
        order_side: OrderSide::Ask,
    };

    group.bench_function(format!("trade market orders").as_str(), |b| {
        b.iter(|| {
            let x = half_book.trade_market_order(black_box(&market_order));
        })
    });

    let limit_order = LimitOrder {
        order_id: 1,
        price: 100,
        quantity: 100,
        order_side: OrderSide::Ask,
    };

    group.bench_function(format!("trade limit orders").as_str(), |b| {
        b.iter(|| {
            let x = half_book.trade_limit_order(black_box(&limit_order));
        })
    });

    group.finish();
}
criterion_group!(
    benches, 
    bench_stress_test,
    //bench_orderbook,
);
criterion_main!(benches);