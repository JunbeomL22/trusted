use trading_engine::types::book_price::BookPrice;
use trading_engine::types::precision::{
    Prec2,
    Precision
};
use ryu;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use anyhow::Result;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestPrice {
    pub price: i64,
}

impl TestPrice {
    #[must_use]
    #[inline]
    pub fn new(price: f64, prec: Precision) -> Result<Self> {
        Ok(
            TestPrice {
                price: prec.price_f64_to_i64(price)?,
            }
        )
    }
}

fn bench_make_book_price(c: &mut Criterion) {
    let mut bgroup = c.benchmark_group("book_price_to_f64");

    let price_str = "123456789.123456789";
    let prec2 = Precision::Prec2;

    bgroup.bench_function("make book price by original implementation", |b| {
        b.iter(|| {
            let val_f64 = price_str.parse::<f64>().unwrap();
            BookPrice::<Prec2>::new(val_f64).expect("Failed to create BookPrice")
        });
    });

    bgroup.bench_function("make book price by enum implementation", |b| {
        b.iter(|| {
            let val_f64 = price_str.parse::<f64>().unwrap();
            TestPrice::new(val_f64, prec2).expect("Failed to create TestPrice")
        });
    });

    bgroup.finish();
}

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

criterion_group!(
    benches, 
    bench_make_book_price,
    bench_book_price);
criterion_main!(benches);
