use trading_engine::types::book_price::BookPrice;
use trading_engine::types::precision::Prec2;
use ryu;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_book_price(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("book_price");

    let price_str = "123456789.123456789";

    let buffer = ryu::Buffer::new();

    bgroup.bench_function("make BookPrice from str", |b| {
        b.iter(|| {
            let price_f64 = price_str.parse::<f64>().unwrap();
            let bp = BookPrice::<Prec2>::new(price_f64).expect("Failed to create BookPrice");
        });
    });

    let price_f64 = price_str.parse::<f64>().unwrap();
    bgroup.bench_function("make BookPrice from f64", |b| {
        b.iter(|| {
            let bp = BookPrice::<Prec2>::new(price_f64).expect("Failed to create BookPrice");
        });
    });

    bgroup.finish();
}

criterion_group!(benches, bench_book_price);
criterion_main!(benches);
