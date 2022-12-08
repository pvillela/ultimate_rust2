use criterion::{black_box, criterion_group, criterion_main, Criterion};
use testing::sploosh;

pub fn sploosh_8_9_10_benchmark(c: &mut Criterion) {
    c.bench_function("sploosh_8_9_10", |b| {
        b.iter(|| sploosh(black_box(8), black_box(9), black_box(10)))
    });
}

criterion_group!(benches, sploosh_8_9_10_benchmark);
criterion_main!(benches);
