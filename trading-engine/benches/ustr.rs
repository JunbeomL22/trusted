use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ustr::{Ustr, ustr as u};
use lazy_format::lazy_format;

fn bench_string(c: &mut Criterion) {

    let mut format_group = c.benchmark_group("Formatting");
    let ustr1 = u("the quick brown fox");
    let ustr2 = u("the quick brown fox");

    let string1 = String::from("the quick brown fox");
    let string2 = String::from("the quick brown fox");

    format_group.bench_function("Ustr::fmt", |b| {
        b.iter(|| format!("{} {}", ustr1, ustr2));
    });

    format_group.bench_function("String::fmt", |b| {
        b.iter(|| format!("{} {}", string1, string2));
    });

    format_group.bench_function("lazy_format ustr and string", |b| {
        b.iter(|| lazy_format!("{} {}", ustr1, ustr2).to_string());
    });

    format_group.finish();

    let mut comparison_group = c.benchmark_group("Writing");
    let ustr1 = u("the quick brown fox");
    let ustr2 = u("the quick brown fox");

    let string1 = String::from("the quick brown fox");
    let string2 = String::from("the quick brown fox");

    comparison_group.bench_function("Ustr::comparison", |b| {
        b.iter(|| {
            black_box(ustr1 == ustr2);
        });
    });

    comparison_group.bench_function("String::comparison", |b| {
        b.iter(|| {
            black_box(string1 == string2);
        });
    });

    comparison_group.finish();

    let mut byte_conversion_group = c.benchmark_group("Byte Conversion");
    let ustr = u("the quick brown fox");
    let string = String::from("the quick brown fox");

    byte_conversion_group.bench_function("Ustr::as_bytes", |b| {
        b.iter(|| ustr.as_bytes());
    });

    byte_conversion_group.bench_function("String::as_bytes", |b| {
        b.iter(|| string.as_bytes());
    });

}

criterion_group!(benches, bench_string);
criterion_main!(benches);

