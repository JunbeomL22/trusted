use trading_engine::orderbook::{
    level::Level,
    half_book::HalfBook,
};
use trading_engine::types::{
    base::{BookQuantity, OrderId, BookPrice},
    enums::OrderSide,
};
use trading_engine::data::order::BookOrder;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_add_order(c: &mut Criterion) {
    let n = 10_000;
    let price_levels = 50;
    let mut order_vec: Vec<BookOrder> = Vec::with_capacity(n+1);
    for i in 0..(n / price_levels) {
        for j in 0..price_levels {
            order_vec.push(BookOrder {
                order_id: i as u64,
                price: (j+1) as i64,
                quantity: (i+1) as u64,
                order_side: OrderSide::Bid,
            });
        }
    }

    let mut half_book = HalfBook::initialize(OrderSide::Bid);
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
                half_book.add_order(black_box(order)).unwrap();
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

criterion_group!(
    benches, 
    bench_add_order,
);
criterion_main!(benches);